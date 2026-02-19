#![feature(inherent_associated_types)]
#![feature(associated_type_defaults)]
#[allow(incomplete_features)]


use std::env;

use anyhow::Result;
use flyer_orm::{DB, Database, Executor, Query, mysql::MySQL, sqlite::SQLite};
use serde::{Serialize, de};
use uuid::Uuid;
// use sqlx::Database;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct User {
    pub uuid: String,
    pub created_at: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}


pub struct Connection;

impl Connection {
    pub type T = SQLite;

    pub fn url<'q>() -> String {
        return match env::var("ENVIRONMENT").unwrap_or("testing".to_string()).as_str() {
            "production"  => env::var("DATABASE_URL").unwrap(), // TODO: fix temp variable...
            "development" => "./database.sqlite".to_string(),
            _             => "./database.sqlite".to_string(), // ":memory:".to_string(),
        }
    }

    pub async fn db() -> Database<Connection::T> {
        return Database::new(&Self::url()).await;
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let db: Database<Connection::T> = Connection::db().await;
    let transaction = db.transaction().await.unwrap();

    let users = db.query("users")
        .insert_as::<User>(vec!["uuid", "first_name", "last_name", "email", "password"])
        .bind(Uuid::new_v4().to_string())
        .bind("Jane")
        .bind("Can")
        .bind("jane@gmail.com")
        .bind("password@123")
        .execute()
        .await
        .unwrap();


    println!("\r\n\r\n ------------ GET USERS ------------ \r\n\r\n {:?} \r\n\r\n\r\n\r\n", users);

    transaction.commit().await.unwrap();
        
    Ok(())
}


//   "uuid": "296598c0-095c-4c88-a48c-8af6c98022ff",
//   "created_at": "2025-11-26 10:07:33",
//   "first_name": "Themba Lucas",
//   "last_name": "Ngubeni",
//   "email": "thembangubeni04@gmail.com",
//   "password": "$2a$10$woMg6Ftrz8DyZCKhvPgMgOrO/YWaZq1JkM8KaAQlOKhBCcrSrboC."