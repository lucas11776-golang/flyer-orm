use flyer_orm::{Database, Entity, Executor, Postgres, Result, SQLite};
use serde::Serialize;

#[derive(Debug, Entity, Serialize)]
pub struct Post {
    pub id: i64,
    pub title: String,
}

const POSTS_TABLE_SCHEMA: &'static str = "
    CREATE TABLE posts (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        title VARCHAR(255) NOT NULL
    );

    INSERT INTO posts 
        (title)
    VALUES
        ('Post 1'),
        ('Post 2'),
        ('Post 3'),
        ('Post 4'),
        ('Post 5');
";

pub fn db() -> Database<Postgres> {
    Database::<Postgres>::connection("default")
}

#[tokio::main]
pub async fn main() -> Result<()> {
    Database::add_connection("default", SQLite::new(":memory:").await?);

    let migrated = db()
        .raw(POSTS_TABLE_SCHEMA)
        .execute()
        .await;

    if let Err(err) = migrated {
        panic!("Migration Error: {:?}", err);
    }

    let result = db()
        .query("posts")
        .paginate::<Post>(2, 2)
        .await;

    if let Err(err) = result {
        panic!("Get User Error: {:?}", err);
    }

    let pagination = result.unwrap();

    println!("Total posts: {}", pagination.total);
    println!("Current page: {}", pagination.page);
    println!("Items on this page:");

    for post in pagination.items {
        println!(" - [{}]: {}", post.id, post.title);
    }

    Ok(())
}