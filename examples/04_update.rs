use flyer_orm::{Database, Entity, Executor, Postgres, Result, SQLite};
use serde::Serialize;

#[derive(Debug, Entity, Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub status: String,
}

pub fn db() -> Database<Postgres> {
    Database::<Postgres>::connection("default")
}

#[tokio::main]
pub async fn main() -> Result<()> {
    Database::add_connection("default", SQLite::new(":memory:").await?);

    // Set up schema and initial data
    db()
        .raw("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, status TEXT)")
        .execute()
        .await
        .unwrap();

    let user = db()
        .insert("users")
        .bind("name", String::from("John"))
        .bind("status", String::from("inactive"))
        .execute_as::<User>()
        .await
        .unwrap();

    println!("\r\n\r\nUser created with status: {}", user.status);

    db()
        .update("users")
        .r#where("id", "=", user.id)
        .bind("status", String::from("active"))
        .execute()
        .await
        .unwrap();

    let user = db()
        .query("users")
        .r#where("id", "=", user.id)
        .first::<User>()
        .await
        .unwrap();

    println!("User updated with status: {}\r\n\r\n", user.status);

    Ok(())
}