use axum::{
    response::{Html, IntoResponse, Response, Redirect},
    extract::{State, Form, Query},
    http::StatusCode,
};
use askama::Template;
use tracing::info;
use axum_extra::extract::cookie::{Cookie, SignedCookieJar};
use uuid::Uuid;
use std::collections::HashMap;

use crate::models::{UserRegister, UserLogin, User, PhraseList, Phrase, PhraseListMetadata};
use axum::extract::{Multipart, Path};
use csv::ReaderBuilder;
use crate::errors::AppError;
use crate::AppState;

const SESSION_COOKIE_NAME: &str = "session_id";

pub async fn root() -> impl IntoResponse {
    let template = IndexTemplate {};
    HtmlTemplate(template)
}

pub async fn get_register() -> impl IntoResponse {
    let template = RegisterTemplate {};
    HtmlTemplate(template)
}

pub async fn post_register(
    State(app_state): State<AppState>,
    Form(form): Form<UserRegister>,
) -> Result<Response, AppError> {
    let hashed_password = bcrypt::hash(form.password, bcrypt::DEFAULT_COST)?;

    let result = sqlx::query!(
        "INSERT INTO users (username, password_hash) VALUES ($1, $2)",
        form.username,
        hashed_password
    )
    .execute(&app_state.pool)
    .await;

    match result {
        Ok(_) => {
            info!("User {} registered successfully", form.username);
            Ok(Html("<h1>Registration successful! <a href=\"/\">Go to Home</a></h1>").into_response())
        }
        Err(e) => {
            tracing::error!("Failed to register user {}: {}", form.username, e);
            if e.to_string().contains("duplicate key value violates unique constraint") {
                Err(AppError::UserAlreadyExists)
            } else {
                Err(AppError::SqlxError(e))
            }
        }
    }
}

pub async fn get_login(
    jar: SignedCookieJar,
) -> impl IntoResponse {
    if jar.get(SESSION_COOKIE_NAME).is_some() {
        Redirect::to("/mypage").into_response()
    } else {
        HtmlTemplate(LoginTemplate {}).into_response()
    }
}

pub async fn post_login(
    State(app_state): State<AppState>,
    jar: SignedCookieJar,
    Form(form): Form<UserLogin>,
) -> Result<(SignedCookieJar, Response), AppError> {
    let user = sqlx::query_as!(User,
        "SELECT id, username, password_hash, created_at as \"created_at!: sqlx::types::chrono::DateTime<chrono::Utc>\" FROM users WHERE username = $1",
        form.username
    )
    .fetch_optional(&app_state.pool)
    .await?;

    match user {
        Some(user_record) => {
            if bcrypt::verify(&form.password, &user_record.password_hash)? {
                info!("User {} logged in successfully", form.username);

                let session_id = Uuid::new_v4().to_string();
                app_state.sessions.lock().unwrap().insert(session_id.clone(), user_record.id);

                let mut cookie = Cookie::new(SESSION_COOKIE_NAME, session_id);
                cookie.set_path("/");
                cookie.set_http_only(true);
                cookie.set_secure(false); // Set to true in production with HTTPS

                Ok((jar.add(cookie), Redirect::to("/mypage").into_response()))
            } else {
                info!("Failed login attempt for user {}", form.username);
                Err(AppError::Unauthorized) // Invalid password
            }
        }
        None => {
            info!("Failed login attempt for user {}", form.username);
            Err(AppError::Unauthorized) // User not found
        }
    }
}

pub async fn logout_user(
    State(app_state): State<AppState>,
    jar: SignedCookieJar,
) -> Result<(SignedCookieJar, Redirect), AppError> {
    if let Some(session_id) = jar.get(SESSION_COOKIE_NAME).map(|c| c.value().to_string()) {
        app_state.sessions.lock().unwrap().remove(&session_id);
    }
    let updated_jar = jar.remove(Cookie::from(SESSION_COOKIE_NAME));
    Ok((updated_jar, Redirect::to("/")))
}

pub async fn my_page(
    State(app_state): State<AppState>,
    jar: SignedCookieJar,
) -> Result<impl IntoResponse, AppError> {
    let session_id = jar
        .get(SESSION_COOKIE_NAME)
        .map(|c| c.value().to_string())
        .ok_or(AppError::Unauthorized)?;

    let user_id = app_state.sessions
        .lock()
        .unwrap()
        .get(&session_id)
        .cloned()
        .ok_or(AppError::Unauthorized)?;

    let user = User::find_by_id(&app_state.pool, user_id).await?;

    let phrase_lists = PhraseList::find_by_user_id(&app_state.pool, user_id).await?;

    let template = MyPageTemplate {
        username: user.username,
        phrase_lists,
    };

    Ok(HtmlTemplate(template))
}

#[derive(Template)]
#[template(path = "mypage.html")]
pub struct MyPageTemplate {
    pub username: String,
    pub phrase_lists: Vec<PhraseList>,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;

#[derive(Template)]
#[template(path = "register.html")]
pub struct RegisterTemplate;

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate;

#[derive(Template)]
#[template(path = "import.html")]
pub struct ImportTemplate;

#[derive(Template)]
#[template(path = "list.html")]
pub struct ListTemplate {
    pub phrases: Vec<Phrase>,
    pub list_id: i32,
    pub display_mode: String,
    pub list: PhraseList,
}

pub async fn get_import() -> impl IntoResponse {
    HtmlTemplate(ImportTemplate {})
}

pub async fn post_import(
    State(app_state): State<AppState>,
    jar: SignedCookieJar,
    mut multipart: Multipart,
) -> Result<Redirect, AppError> {
    let session_id = jar
        .get(SESSION_COOKIE_NAME)
        .map(|c| c.value().to_string())
        .ok_or(AppError::Unauthorized)?;

    let user_id = app_state.sessions
        .lock()
        .unwrap()
        .get(&session_id)
        .cloned()
        .ok_or(AppError::Unauthorized)?;

    let mut list_name = String::new();
    let mut target_lang = String::new();
    let mut source_lang = String::new();
    let mut file_data = Vec::new();

    while let Some(field) = multipart.next_field().await? {
        let name = field.name().unwrap_or("").to_string();
        let data = field.bytes().await?;

        match name.as_str() {
            "name" => list_name = String::from_utf8(data.to_vec()).unwrap_or_default(),
            "target_lang" => target_lang = String::from_utf8(data.to_vec()).unwrap_or_default(),
            "source_lang" => source_lang = String::from_utf8(data.to_vec()).unwrap_or_default(),
            "file" => file_data = data.to_vec(),
            _ => (),
        }
    }

    let mut tx = app_state.pool.begin().await?;

    let list_id = match PhraseList::find_by_name_and_user_id(&mut tx, &list_name, user_id).await? {
        Some(existing_list) => existing_list.id,
        None => {
            let metadata = PhraseListMetadata {
                user_id,
                name: list_name,
                target_lang,
                source_lang,
            };
            PhraseList::create(&mut tx, metadata).await?
        }
    };

    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file_data.as_slice());

    for result in rdr.records() {
        let record = result?;
        let phrase = Phrase {
            id: 0, // DB will generate this
            list_id,
            target_lang_text: record.get(0).unwrap_or("").to_string(),
            source_lang_text: record.get(1).unwrap_or("").to_string(),
        };
        Phrase::create(&mut tx, phrase).await?;
    }

    tx.commit().await?;

    Ok(Redirect::to("/mypage"))
}

pub async fn get_list(
    State(app_state): State<AppState>,
    jar: SignedCookieJar,
    Path(list_id): Path<i32>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, AppError> {
    let session_id = jar
        .get(SESSION_COOKIE_NAME)
        .map(|c| c.value().to_string())
        .ok_or(AppError::Unauthorized)?;

    let user_id = app_state.sessions
        .lock()
        .unwrap()
        .get(&session_id)
        .cloned()
        .ok_or(AppError::Unauthorized)?;

    let list = PhraseList::find_by_id(&app_state.pool, list_id).await?;

    // Ensure the user owns the phrase list
    if list.user_id != user_id {
        return Err(AppError::Unauthorized);
    }

    let phrases = Phrase::find_by_list_id(&app_state.pool, list_id).await?;

    let display_mode = params.get("display_mode").cloned().unwrap_or_else(|| "all".to_string());

    let template = ListTemplate {
        phrases,
        list_id,
        display_mode,
        list,
    };

    Ok(HtmlTemplate(template))
}

// Helper for rendering Askama templates into a response.
pub struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

