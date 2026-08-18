use anyhow::Result;
use flyer_orm::{Database, Entity, Executor, SQLite};

#[derive(Debug, Entity)]
pub struct Projects {
    pub uuid: String,
    pub organization_uuid: String,
    pub name: String,
    pub description: String,
    pub prompts: i64,
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

    let project = db()
        .raw_read("sql/projects.sql")
        .bind(String::from("296598c0-095c-4c88-a48c-8af6c98022ff"))
        .first::<Projects>()
        .await
        .unwrap();

    println!("USER -> {:?}", project);
    
    Ok(())
}