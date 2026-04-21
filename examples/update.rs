use anyhow::Result;
use flyer_orm::{
    Database,
    databases::sqlite::SQLite
};
use serde::Serialize;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub status: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let db = Database::<SQLite>::new(":memory:").await;

    // Set up schema and initial data
    db.execute("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, status TEXT)").await?;
    db.execute("INSERT INTO users (name, status) VALUES ('John', 'inactive')").await?;

    println!("User created with status: inactive");

    // Perform an update
    db.query("users")
        .update(vec!["status"])
        .r#where("name", "=", "John")
        .bind("active")
        .execute()
        .await?;

    // Verify the update
    let updated_user = db.query("users")
        .r#where("name", "=", "John")
        .first::<User>()
        .await?;

    println!("Updated User: {:?}", updated_user);

    return Ok(());
}
