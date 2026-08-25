use flyer_orm::{Database, Entity, Executor, Postgres, Result, SQLite};
use serde::Serialize;

#[derive(Debug, Entity, Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
}

pub fn db() -> Database<Postgres> {
    Database::<Postgres>::connection("default")
}

#[tokio::main]
pub async fn main() -> Result<()> {
    Database::add_connection("default", SQLite::new(":memory:").await?);

    // Set up schema and initial data
    db()
        .raw("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, email TEXT)")
        .execute()
        .await
        .unwrap();

    /* INSERT DATA */
    db()
        .insert("users")
        .bind("name", String::from("John"))
        .bind("email", String::from("john@deo.com"))
        .execute()
        .await
        .unwrap();

    db()
        .insert("users")
        .bind("name", String::from("Jane"))
        .bind("email", String::from("jane@doe.com"))
        .execute()
        .await
        .unwrap();

    println!("\r\n\r\nNumber of users: {}", db()
        .query("users").count().await.unwrap());

    db()
        .delete("users")
        .r#where("email", "=",String::from("john@deo.com"))
        .execute()
        .await
        .unwrap();

    let total = db()
        .query("users")
        .count()
        .await
        .unwrap(); 

    println!("Number of users: {}\r\n\r\n", total);

    Ok(())
}