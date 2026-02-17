#![feature(inherent_associated_types)]
use std::{env, marker::PhantomData};

use anyhow::Result;
use flyer_orm::{DB, Query, mysql::MySQL, sqlite::SQLite};
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


pub struct Database;

impl Database {
    pub type T = SQLite;
    // pub type T = MySQL::<'q>;

    pub fn url<'q>() -> &'q str {
        return match env::var("ENVIRONMENT").unwrap_or("testing".to_string()).as_str() {
            // "production"  => env::var("DATABASE_URL").unwrap().as_str(), // TODO: fix temp variable...
            "development" => "./database.sqlite",
            _             => "./database.sqlite", // ":memory:".to_string(),
        }
    }

    pub fn query<'q>() -> Query<'q, Database::T> {
        return DB::query_url::< Database::T>(Self::url());
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    DB::init();
    DB::add("sqlite", "./database.sqlite");

    let users = Database::query()
        .table("users")
        .select(vec!["*"])
        .r#where("email", "=", "thembangubeni04@gmail.com")
        .get::<User>(10)
        .await
        .unwrap();

    println!("\r\n\r\n ------------ GET USERS ------------ \r\n\r\n {:?} \r\n\r\n\r\n\r\n", users);

    let users = Database::query()
        .table("users")
        .select(vec!["*"])
        .r#where("email", "=", "thembangubeni04@gmail.com")
        .paginate::<User>(10, 1)
        .await
        .unwrap();

    println!("\r\n\r\n ------------ PAGINATE USERS ------------ \r\n\r\n {:?} \r\n\r\n\r\n\r\n", users);
        
    Ok(())
}