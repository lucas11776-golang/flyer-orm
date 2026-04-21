use std::env;

use anyhow::Result;
use flyer_orm::{Database, databases::{postgres::Postgres, sqlite::SQLite}};


// use sqlx::postgres::types::


#[derive(Debug, sqlx::FromRow)]
pub struct Project {
    // pub id: String,
    pub name: String
}



#[derive(Debug, sqlx::FromRow)]
pub struct User {
    // pub id: Uuid,
    pub email: String
}


pub struct Connection;

impl Connection {
    // pub async fn db() -> Database<SQLite> {
    //     return Database::<SQLite>::new("./database.sqlite").await;
    // }
    pub async fn db() -> Database<Postgres> {
        return Database::<Postgres>::new(env::var("DATABASE_URL").unwrap().as_str()).await;
    }
}

#[tokio::main]

 async fn main() -> Result<()> {
    dotenv::from_path(".env").unwrap();

    let db = Connection::db().await;

    db.query("users")
        .update(vec!["first_name", "last_name"])
        .bind("Jeo")
        .bind("Deo")
        .r#where("email", "=", "thembangubeni04@gmail.com")
        .execute()
        .await
        .unwrap();

    Ok(())
}