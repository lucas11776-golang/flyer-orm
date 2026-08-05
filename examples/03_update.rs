use flyer_orm::Result;
use flyer_orm::{Connection, Executor, Query, SQLite, query::raw::Raw};
use flyer_orm::Entity;
use serde::Serialize;


#[derive(Debug, Entity, Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub status: String,
}

pub struct Database;

impl Database {
    pub fn query<'q>(table: impl Into<String>) -> Query<'q, SQLite> {
        Connection::query("default").table(table)
    }

    pub fn raw<'q>(sql: impl Into<String>) -> Raw<'q, SQLite> {
        Connection::raw("default", sql)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    Connection::add("default", SQLite::new(":memory:").await);

    // Set up schema and initial data
    Database::raw("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, status TEXT)")
        .execute()
        .await
        .unwrap();

    let user = Database::query("users")
        .insert()
        .bind("name", String::from("John"))
        .bind("status", String::from("inactive"))
        .execute_as::<User>()
        .await
        .unwrap();

    println!("\r\n\r\nUser created with status: {}", user.status);

    Database::query("users")
        .update()
        .r#where("id", "=", user.id)
        .bind("status", String::from("active"))
        .execute()
        .await
        .unwrap();

    let user = Database::query("users")
        .r#where("id", "=", user.id)
        .first::<User>()
        .await
        .unwrap();

    println!("User updated with status: {}\r\n\r\n", user.status);

    return Ok(());
}