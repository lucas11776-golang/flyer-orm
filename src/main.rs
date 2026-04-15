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
    // pub async fn db() -> Database<Postgres> {
    //     return Database::<Postgres>::new(env::var("DATABASE_URL").unwrap().as_str()).await;
    // }
    pub async fn db() -> Database<SQLite> {
        return Database::<SQLite>::new("./database.sqlite").await;
    }
}

#[tokio::main]

 async fn main() -> Result<()> {
    dotenv::from_path(".env").unwrap();

    let db = Connection::db().await;

    // let user = db.query("users")
    //     // .select(vec!["cities.*", "ST_AsText(location::text) AS location"])
    //     .select(vec!["*"])
    //     // .r#where("role", ">=", 2)
    //     // .and_where("role", "<=", 5)
    //     .r#where("email", "LIKE", "gmail.com")
    //     .paginate::<User>(1, 1)
    //     .await
    //     .unwrap();

    // println!("SQL -> {:?}", user);


    let projects = db.query("projects")
        .select(vec!["name"])
        .paginate::<Project>(1, 2)
        .await
        .unwrap();


    println!("PROJECT -> {:?}", projects);

    Ok(())
}