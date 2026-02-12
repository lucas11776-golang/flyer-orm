use anyhow::Result;
use flyer_orm::{DB, query::Order, sqlite::SQLite};
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

#[tokio::main]
async fn main() -> Result<()> {
    DB::add("sqlite", SQLite::connect("./database.sqlite").await.unwrap());

    let users = DB::query("sqlite")
        .table("users")
        .limit(10)
        .order_by("created_at", Order::DESC)
        .get::<User>(20)
        .await
        .unwrap();

    println!("{:?}", users);

    Ok(())
}