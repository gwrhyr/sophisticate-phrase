use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Postgres, Transaction, PgPool};

// --- User Models ---

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub created_at: sqlx::types::chrono::DateTime<chrono::Utc>,
}

impl User {
    pub async fn find_by_id(
        pool: &PgPool,
        id: i32,
    ) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, username, password_hash, created_at as "created_at!: chrono::DateTime<chrono::Utc>"
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
    }
}



#[derive(Debug, Deserialize)]
pub struct UserRegister {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

// --- Phrase List Models ---


#[derive(Debug, FromRow, Serialize)]
pub struct PhraseList {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub target_lang: String,
    pub source_lang: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl PhraseList {
    pub async fn create(
        transaction: &mut Transaction<'_, Postgres>,
        metadata: PhraseListMetadata,
    ) -> Result<i32, sqlx::Error> {
        let list_id = sqlx::query!(
            r#"
            INSERT INTO phrase_lists (user_id, name, target_lang, source_lang)
            VALUES ($1, $2, $3, $4)
            RETURNING id
            "#,
            metadata.user_id,
            metadata.name,
            metadata.target_lang,
            metadata.source_lang,
        )
        .fetch_one(&mut **transaction)
        .await?
        .id;

        Ok(list_id)
    }

    pub async fn find_by_user_id(
        pool: &PgPool,
        user_id: i32,
    ) -> Result<Vec<PhraseList>, sqlx::Error> {
        sqlx::query_as!(
            PhraseList,
            r#"
            SELECT id, user_id, name, target_lang, source_lang, created_at
            FROM phrase_lists
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(pool)
        .await
    }

    pub async fn find_by_id(
        pool: &PgPool,
        id: i32,
    ) -> Result<PhraseList, sqlx::Error> {
        sqlx::query_as!(
            PhraseList,
            r#"
            SELECT id, user_id, name, target_lang, source_lang, created_at
            FROM phrase_lists
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_name_and_user_id(
        transaction: &mut Transaction<'_, Postgres>,
        name: &str,
        user_id: i32,
    ) -> Result<Option<PhraseList>, sqlx::Error> {
        sqlx::query_as!(
            PhraseList,
            r#"
            SELECT id, user_id, name, target_lang, source_lang, created_at
            FROM phrase_lists
            WHERE name = $1 AND user_id = $2
            "#,
            name,
            user_id
        )
        .fetch_optional(&mut **transaction)
        .await
    }
}

#[derive(Debug)]
pub struct PhraseListMetadata {
    pub user_id: i32,
    pub name: String,
    pub target_lang: String,
    pub source_lang: String,
}

// --- Phrase Models ---

#[derive(Debug, FromRow, Serialize)]
pub struct Phrase {
    pub id: i32,
    pub list_id: i32,
    pub target_lang_text: String,
    pub source_lang_text: String,
}

impl Phrase {
    pub async fn create(
        transaction: &mut Transaction<'_, Postgres>,
        phrase: Phrase,
    ) -> Result<i32, sqlx::Error> {
        let phrase_id = sqlx::query!(
            r#"
            INSERT INTO phrases (list_id, target_lang_text, source_lang_text)
            VALUES ($1, $2, $3)
            RETURNING id
            "#,
            phrase.list_id,
            phrase.target_lang_text,
            phrase.source_lang_text,
        )
        .fetch_one(&mut **transaction)
        .await?
        .id;

        Ok(phrase_id)
    }

    pub async fn find_by_list_id(
        pool: &PgPool,
        list_id: i32,
    ) -> Result<Vec<Phrase>, sqlx::Error> {
        sqlx::query_as!(
            Phrase,
            r#"
            SELECT id, list_id, target_lang_text, source_lang_text
            FROM phrases
            WHERE list_id = $1
            ORDER BY id ASC
            "#,
            list_id
        )
        .fetch_all(pool)
        .await
    }
}

// --- Display Models ---

#[derive(Debug, Serialize)]
pub struct PhraseListDisplay {
    pub name: String,
    pub phrases: Vec<Phrase>,
    pub display_mode: String,
}