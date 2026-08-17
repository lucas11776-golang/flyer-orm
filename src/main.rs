use anyhow::Result;
use flyer_orm::{Database, Entity, Executor, SQLite};

#[derive(Debug, Entity)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

pub fn db<'q>() -> Database<'q, SQLite> {
    Database::<SQLite>::connection("default")
}

pub async fn destroy(id: String) -> Result<()> {
    db()
        .delete("users")
        .r#where("id", "=", id)
        .execute()
        .await
}

#[tokio::main]
async fn main() -> Result<()> {
    Database::add("default", SQLite::new("database.sqlite").await);

    let user = db()
        .query("users")
        .first::<User>()
        .await
        .unwrap();

    println!("USER -> {:?}", user);

    destroy(String::from("n-296598c0-095c-4c88-a48c-8af6c98022ff"))
        .await
        .unwrap();
    
    Ok(())
}