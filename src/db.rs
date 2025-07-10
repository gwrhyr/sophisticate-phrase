use sqlx::PgPool;
use tracing::info;

pub async fn setup_database() -> Result<PgPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;
    info!("Running migrations...");
    sqlx::migrate!().run(&pool).await?;
    info!("Migrations finished.");
    Ok(pool)
}