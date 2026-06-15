use anyhow::Result;
use flyer_orm::{Database, databases::{sqlite::SQLite}};
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

const USERS_TABLE_SCHEME: &'static str = "CREATE TABLE users (
 `uuid` VARCHAR(65535) PRIMARY KEY NOT NULL UNIQUE,
 `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 `first_name` VARCHAR(65535),
 `last_name` VARCHAR(65535),
 `email` VARCHAR(65535) NOT NULL,
 `password` VARCHAR(65535) NOT NULL
)";

pub struct Connection;

impl Connection {
    // TODO: Using SQLite
    pub async fn db() -> Database<SQLite> {
        return Database::<SQLite>::new(":memory:").await;
    }

    // // TODO: Using Postgres
    // pub async fn db() -> Database<Postgres> {
    //     return Database::<Postgres>::new(&env::var("DATABASE_URL").unwrap()).await;
    // }

    // // TODO: Using MySQL
    // pub async fn db() -> Database<MySQL> {
    //     return Database::<MySQL>::new(&env::var("DATABASE_URL").unwrap()).await;
    // }
}

#[tokio::main]
async fn main() -> Result<()> {
    let db = Connection::db().await;

    // Migrate database with users table
    if let Err(err) = db.raw_query(USERS_TABLE_SCHEME).execute().await {
        panic!("Error: {:?}", err)
    }

    let user = db.query("users")
        .insert_as::<User>(vec!["uuid", "first_name", "last_name", "email", "password"])
        .bind(uuid::Uuid::new_v4().to_string())
        .bind("Jeo")
        .bind("doe")
        .bind("jeo@doe.com")
        .bind("test@123")
        .execute()
        .await
        .unwrap();

    println!("User: {:?}", user);

    return Ok(());
}