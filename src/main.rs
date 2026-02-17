#![feature(inherent_associated_types)]
#[allow(incomplete_features)]

use std::env;

use anyhow::Result;
use flyer_orm::{DB, Database, Query, sqlite::SQLite};
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


#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Project {
    pub uuid: String,
    pub organization_uuid: String,
    pub user_uuid: String,
    pub container_id: String,
    pub created_at: String,
    pub ip_address: String,
    pub name: String,
    pub framework: String,
    pub model: String,
    pub description: String,
}

pub struct Connection;

impl Connection {
    pub type T = SQLite;

    pub fn url<'q>() -> &'q str {
        return match env::var("ENVIRONMENT").unwrap_or("testing".to_string()).as_str() {
            // "production"  => env::var("DATABASE_URL").unwrap().as_str(), // TODO: fix temp variable...
            "development" => "./database.sqlite",
            _             => "./database.sqlite", // ":memory:".to_string(),
        }
    }

    pub async fn db() -> Database<Connection::T> {
        return Database::new(Self::url()).await;
    }

    // pub async fn query<'q>() -> Query<'q, Connection::T> {
    //     return DB::query_url::< Database::T>(Self::url()).await;
    // }
}

#[tokio::main]
async fn main() -> Result<()> {
    DB::add("sqlite", "./database.sqlite");

    // let projects = Database::query()
    //     .await
    //     .table("users")
    //     .select(vec!["projects.*"])
    //     .join("projects", "users.uuid", "projects.user_uuid")
    //     .all::<Project>()
    //     .await
    //     .unwrap();

    // println!("\r\n\r\n ------------ GET USERS ------------ \r\n\r\n {:?} \r\n\r\n\r\n\r\n", projects);
        
    Ok(())
}