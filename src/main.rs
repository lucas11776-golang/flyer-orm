#![feature(inherent_associated_types)]
#![feature(associated_type_defaults)]
#[allow(incomplete_features)]

use std::env;
use std::{fs::File, io::Write};

use anyhow::Result;
use flyer_orm::{Database, databases::sqlite::SQLite, query::Order};
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
    pub created_at: String,
    pub organization_uuid: String,
    pub user_uuid: String,
    pub container_id: String,
    pub ip_address: String,
    pub name: String,
    pub framework: String,
    pub model: String,
    pub description: String,
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

    // db.query("projects")
    //     .update(vec!["model"])
    //     .r#where("uuid", "=", "296598c0-095c-4c88-a48c-8af6c98022ff")
    //     .bind("gemini")
    //     .execute()
    //     .await
    //     .unwrap();

    


    // let sql = db.query("users")
    //     .r#where("uuid", "=", "296598c0-095c-4c88-a48c-8af6c98022ff")
    //     .and_where("first_name", "=", "Jane")
    //     .order_by("created_at", Order::ASC)
    //     .order_by("first_name", Order::DESC)
    //     .all::<User>()
    //     .await
    //     // .to_sql()
    //     .unwrap();

    // println!("SQL -> {:?}", sql);


    

    // let user = db.query("users")
    //     .insert_as::<User>(vec!["uuid", "first_name", "last_name", "email", "password"])
    //     .bind("296598c0-095c-4c88-a48c-8af6c98022fe")
    //     .bind("Lucas")
    //     .bind("Themba Lucas")
    //     .bind("Ngubeni")
    //     .bind("thembangubeni05@gmail.com")
    //     .bind("$2a$10$woMg6Ftrz8DyZCKhvPgMgOrO/YWaZq1JkM8KaAQlOKhBCcrSrboC.")
    //     .execute()
    //     .await
    //     .unwrap();

    // println!("{:?}", user);

    


    let projects = db.query("users")
        .select(vec!["projects.*"])
        .r#where("users.uuid", "=", "296598c0-095c-4c88-a48c-8af6c98022ff")
        .order_by("users.created_at", Order::ASC)
        .join("projects", "users.uuid", "=", "projects.user_uuid")
        .all::<Project>()
        .await
        // .to_sql()
        .unwrap();

    File::create("projects.json")
        .unwrap()
        .write_all(serde_json::to_string_pretty(&projects).unwrap().as_bytes())
        .unwrap();

    println!("SQL -> {:?}", projects);

    transaction.commit().await.unwrap();
        
    Ok(())
}


//   "uuid": "296598c0-095c-4c88-a48c-8af6c98022ff",
//   "created_at": "2025-11-26 10:07:33",
//   "first_name": "Themba Lucas",
//   "last_name": "Ngubeni",
//   "email": "thembangubeni04@gmail.com",
//   "password": "$2a$10$woMg6Ftrz8DyZCKhvPgMgOrO/YWaZq1JkM8KaAQlOKhBCcrSrboC."