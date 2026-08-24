
use flyer_orm::{Database, Entity, Executor, Result, SQLite};

const USERS_TABLE_SCHEME: &'static str = "CREATE TABLE users (
 `uuid` VARCHAR(65535) PRIMARY KEY NOT NULL UNIQUE,
 `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 `first_name` VARCHAR(65535),
 `last_name` VARCHAR(65535),
 `email` VARCHAR(65535) NOT NULL,
 `password` VARCHAR(65535) NOT NULL
)";

#[derive(Debug, Entity)]
pub struct User {
    pub uuid: String,
    pub created_at: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

pub fn db() -> Database<SQLite> {
    Database::<SQLite>::connection(":memory:")
}

#[tokio::main]
async fn main() -> Result<()> {
    Database::add_connection("default", SQLite::new(":memory:").await?);

    let migrated = db()
        .raw(USERS_TABLE_SCHEME)
        .execute()
        .await;

    if let Err(err) = migrated {
        panic!("Migration Error: {:?}", err);
    }

    let result = db()
        .insert("users")
        .bind("uuid", uuid::Uuid::new_v4().to_string())
        .bind("first_name", "Jeo".to_string())
        .bind("last_name", "Deo".to_string())
        .bind("email", "jeo@deo.com".to_string())
        .bind("password", "test@123".to_string())
        .execute_as::<User>()
        .await;

    if let Err(err) = result {
        panic!("Get User Error: {:?}", err);
    }

    let user = result.unwrap();

    println!("\r\n\r\n\r\n\r\nUser: {:?}\r\n\r\n\r\n\r\n", user);

    return Ok(());
}