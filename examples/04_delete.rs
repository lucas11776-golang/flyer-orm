use anyhow::Result;
use flyer_orm::{Connection, Executor, Query, SQLite, query::raw::Raw};
use flyer_orm::Entity;
use serde::Serialize;

#[derive(Debug, Entity, Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
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
    Database::raw("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, email TEXT)")
        .execute()
        .await
        .unwrap();

    /* INSERT DATA */
    Database::query("users")
        .insert()
        .bind("name", String::from("John"))
        .bind("email", String::from("john@deo.com"))
        .execute()
        .await
        .unwrap();

    Database::query("users")
        .insert()
        .bind("name", String::from("Jane"))
        .bind("email", String::from("jane@doe.com"))
        .execute()
        .await
        .unwrap();

    println!("\r\n\r\nNumber of users: {}", Database::query("users").count().await.unwrap());

    Database::query("users")
        .delete()
        .r#where("email", "=",String::from("john@deo.com"))
        .execute()
        .await
        .unwrap();

    println!("Number of users: {}\r\n\r\n", Database::query("users").count().await.unwrap());

    return Ok(());
}