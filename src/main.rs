use axum::{routing::{get, post, get_service}, Router};
use tower_http::services::ServeFile;
use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
use sqlx::PgPool;
use std::{collections::HashMap, sync::{Arc, Mutex}};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod models;
mod handlers;
mod errors;
mod db;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub sessions: Arc<Mutex<HashMap<String, i32>>>,
    pub secret_key: Key,
}

#[tokio::main]
async fn main() {
    // Initialize tracing for logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "sophisticate_phrase=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Database setup
    let pool = db::setup_database().await.expect("Failed to setup database");

    // Session management setup
    let sessions: Arc<Mutex<HashMap<String, i32>>> = Arc::new(Mutex::new(HashMap::new()));
    let secret_key = Key::generate(); // In a real app, load from secure env var

    let app_state = AppState {
        pool,
        sessions,
        secret_key,
    };

    // build our application with routes
    let app = Router::new()
        .route("/", get(handlers::root))
        .route("/register", get(handlers::get_register).post(handlers::post_register))
        .route("/login", get(handlers::get_login).post(handlers::post_login))
        .route("/mypage", get(handlers::my_page))
        .route("/import", get(handlers::get_import).post(handlers::post_import))
        .route("/list/:id", get(handlers::get_list))
        .route("/logout", post(handlers::logout_user))
        .with_state(app_state)
        .route("/output.css", get_service(ServeFile::new("output.css")))
        .route("/images/Gemini_Generated_Image_uuchy3uuchy3uuch.png", get_service(ServeFile::new("images/Gemini_Generated_Image_uuchy3uuchy3uuch.png")));

    // run it with hyper on `localhost:3000`
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

impl FromRef<AppState> for Key {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.secret_key.clone()
    }
}
