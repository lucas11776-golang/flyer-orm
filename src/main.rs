use anyhow::Result;
use flyer_orm::{Database, Entity, Executor, Postgres};
use serde::{Deserialize, Serialize};

#[derive(Debug, Entity)]
pub struct Projects {
    pub uuid: String,
    pub organization_uuid: String,
    pub name: String,
    pub description: String,
    pub prompts: i64,
}

#[derive(Debug, Entity, Serialize, Deserialize)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

pub fn db() -> Database<Postgres> {
    Database::<Postgres>::connection("default")
}

#[tokio::main]
async fn main() -> Result<()> {
    // Database::add("default", SQLite::new("database.sqlite").await?);
    Database::add_connection("default", Postgres::new("postgresql://postgres:test123@localhost:5111/lucas11776").await?);

    let user: sqlx::types::Json<User> = db()
        .scalar_from_file("sql/users.sql")
        .first()
        .await
        .unwrap();

    println!("\r\n\r\nUSER JSON -> {:?}\r\n\r\n", user);

    let user: User = db()
        .query("users")
        .first()
        .await
        .unwrap();

    println!("USER ENTITY -> {:?}\r\n\r\n", user);

    
    Ok(())
}