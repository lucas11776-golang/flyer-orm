use std::env;

use anyhow::Result;
use flyer_orm::{Database, databases::postgres::Postgres};
use sqlx::postgres::types::{PgPoint};
use uuid::Uuid;


// use sqlx::postgres::types::


#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String
}

pub struct Connection;

impl Connection {
    pub async fn db() -> Database<Postgres> {
        return Database::<Postgres>::new(env::var("DATABASE_URL").unwrap().as_str()).await;
    }
}

#[tokio::main]

 async fn main() -> Result<()> {
    dotenv::from_path(".env").unwrap();

    let db = Connection::db().await;

    let user = db.query("users")
        // .select(vec!["cities.*", "ST_AsText(location::text) AS location"])
        .select(vec!["*"])
        .r#where("role", ">=", 2)
        .and_where("role", "<=", 5)
        .all::<User>()
        .await
        .unwrap();

    println!("SQL -> {:?}", user);


    // let cities = db.query("users")
    //     // .select(vec!["cities.*", "ST_AsText(location::text) AS location"])
    //     .select(vec!["cities.*", "location::point as location"])
    //     .r#where("role", ">=", 2)
    //     .all::<User>()
    //     .await
    //     .unwrap();

    // println!("CITIES -> {:?}", cities);

    Ok(())
}