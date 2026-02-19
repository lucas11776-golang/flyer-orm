use anyhow::Result;
use flyer_orm::{DB, sqlite::SQLite};
use serde::Serialize;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct User {
    pub uuid: String,
    pub created_at: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

const TABLE: &'static str = "CREATE TABLE users (
 `uuid` VARCHAR(65535) PRIMARY KEY NOT NULL UNIQUE,
 `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 `first_name` VARCHAR(65535),
 `last_name` VARCHAR(65535),
 `email` VARCHAR(65535) NOT NULL,
 `password` VARCHAR(65535) NOT NULL
)";


#[tokio::main]
async fn main() -> Result<()> {
    let db = DB::db_with_url::<SQLite>(":memory:").await;


    // db.query()



    return Ok(());
}