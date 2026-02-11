use std::{fs::File, io::Write};

use anyhow::{Ok, Result};
use serde::Serialize;
use sqlx::{FromRow, Sqlite, SqlitePool, sqlite::SqliteRow};

#[derive(Serialize)]
pub struct Pagination<Entity> {
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
    pub items: Vec<Entity>
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct User {
    pub uuid: String,
    pub created_at: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

pub struct Query {
    database: sqlx::Pool<Sqlite>,
}

impl Query {
    pub async fn new(database_url: &str) -> Result<Self> {
        return Ok(Self {
            database: SqlitePool::connect(database_url).await.unwrap()
        });
    }

    pub fn select(&mut self, _fields: Vec<&'static str>) -> &mut Self {
        // Hold
        return self;
    }

    pub fn from(&mut self, _key: &str, _value: &str) -> &mut Self {
        // Hold
        return self;
    }

    pub async fn all<'q, O>(&self) -> Vec<O>
    where
        O: for<'r> FromRow<'r, SqliteRow> + Send + Unpin,
    {
        return sqlx::query_as::<_, O>(self.sql())
            .fetch_all(&self.database)
            .await
            .unwrap();
    }

    pub async fn first<'q, O>(&self) -> O
    where
        O: for<'r> FromRow<'r, SqliteRow> + Send + Unpin,
    {
        return sqlx::query_as::<_, O>(self.sql())
            .fetch_one(&self.database)
            .await
            .unwrap();
    }

    pub async fn paginate<'q, O>(&self, limit: u64, page: u64) -> Pagination<O>
    where
        O: for<'r> FromRow<'r, SqliteRow> + Send + Unpin,
    {
        return Pagination {
            total: 1,
            page: page,
            per_page: limit,
            items: Vec::new(),
        };
    }

    fn sql(&self) -> &str {
        return "SELECT * FROM `users` WHERE 1";
    }
}

#[tokio::main]
async fn main() -> Result<()> {
        let query = Query::new("database.sqlite").await.unwrap();

        let users = query.first::<User>().await;
        
        let mut file = File::create("users.json").unwrap();

        file.write_all(serde_json::to_string(&users).unwrap().as_bytes()).unwrap();

        println!("{:?}", users);

    Ok(())
}