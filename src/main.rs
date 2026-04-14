use std::env;

use anyhow::Result;
use flyer_orm::{Database, databases::postgres::Postgres};
use sqlx::postgres::types::PgPoint;

#[derive(Debug, sqlx::FromRow)]
pub struct City {
    pub id: i32,
    pub name: String,
    pub location: PgPoint
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

    let cities = db.query("cities")
        // .select(vec!["cities.*", "ST_AsText(location::text) AS location"])
        .select(vec!["cities.*", "location::point as location"])
        .all::<City>()
        .await
        .unwrap();

    println!("CITIES -> {:?}", cities);

    Ok(())
}