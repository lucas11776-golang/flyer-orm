#![feature(inherent_associated_types)]
#[allow(incomplete_features)]

use std::env;

use anyhow::Result;
use flyer_orm::{DB, Query, sqlite::SQLite};
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
    DB::add("sqlite", "./database.sqlite");

    // let users = Database::query()
    //     .table("users")
    //     .select(vec!["*"])
    //     .r#where("email", "=", "thembangubeni04@gmail.com")
    //     .get::<User>(10)
    //     .await
    //     .unwrap();


    // let sql = Database::query()
    //     .table("users")
    //     .select(vec!["*"])
    //     .r#where("email", "=", "thembangubeni04@gmail.com")
    //     .limit(10)
    //     .to_sql()
    //     .unwrap();

    let sql = Database::query()
        .table("users")
        .query_one::<User, _>("SELECT * FROM `users` WHERE email = ?", vec!["thembangubeni04@gmail.com"])
        .await
        .unwrap();

    println!("\r\n\r\n ------------ SQL USERS ------------ \r\n\r\n {:?} \r\n\r\n\r\n\r\n", sql);

    // println!("\r\n\r\n ------------ GET USERS ------------ \r\n\r\n {:?} \r\n\r\n\r\n\r\n", users);

    // let users = Database::query()
    //     .table("users")
    //     .select(vec!["*"])
    //     .r#where("email", "=", "thembangubeni04@gmail.com")
    //     .paginate::<User>(10, 1)
    //     .await
    //     .unwrap();

    // println!("\r\n\r\n ------------ PAGINATE USERS ------------ \r\n\r\n {:?} \r\n\r\n\r\n\r\n", users);
        
    Ok(())
}