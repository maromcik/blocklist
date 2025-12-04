use crate::database::error::DbError;
use diesel::Connection;
use diesel::PgConnection;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::bb8::Pool;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use log::info;
use std::env;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub type Id = i64;

pub async fn init() -> Result<Pool<AsyncPgConnection>, DbError> {
    run_migrations()?;
    set_up_database_pool().await
}

pub fn run_migrations() -> Result<(), DbError> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut connection = PgConnection::establish(&database_url)?;

    connection
        .run_pending_migrations(MIGRATIONS)
        .map_err(|e| DbError::MigrationError(e.to_string()))?;
    info!("Migrations ran successfully.");
    Ok(())
}

async fn set_up_database_pool() -> Result<Pool<AsyncPgConnection>, DbError> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    let pool = Pool::builder().max_size(20).build(config).await?;
    Ok(pool)
}
