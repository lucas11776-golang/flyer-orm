use anyhow::Result;
use flyer_orm::{DB, sqlite::SQLite};
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
    DB::init();
    DB::add("sqlite", "./database.sqlite");

    let users = DB::query::<SQLite>("sqlite")
        .table("users")
        .select(vec!["*"])
        .get::<User>(10)
        .await
        .unwrap();

    println!("{:?}", users);
        
    Ok(())
}