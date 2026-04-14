use anyhow::Result;
use flyer_orm::{
    Database,
    databases::sqlite::SQLite
};
use serde::Serialize;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Post {
    pub id: i64,
    pub title: String,
}

const SCHEMA: &'static str = "
CREATE TABLE posts (
    id INTEGER PRIMARY KEY AUTO_INCREMENT,
    title VARCHAR(255) NOT NULL
);

INSERT INTO posts (title) VALUES ('Post 1'), ('Post 2'), ('Post 3'), ('Post 4'), ('Post 5');
";

#[tokio::main]
async fn main() -> Result<()> {
    // Using SQLite memory database for example
    let db = Database::<SQLite>::new(":memory:").await;

    // Run schema
    db.execute(SCHEMA).await?;

    // Paginate results: 2 items per page, page 2
    let page = 2;
    let per_page = 2;
    
    let pagination = db.query("posts")
        .paginate::<Post>(per_page, page)
        .await?;

    println!("Total posts: {}", pagination.total);
    println!("Current page: {}", pagination.page);
    println!("Items on this page:");
    for post in pagination.items {
        println!(" - [{}]: {}", post.id, post.title);
    }

    return Ok(());
}
