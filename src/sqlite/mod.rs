mod builder;

use anyhow::Result;
use sqlx::{FromRow, Pool, SqlitePool};

use crate::{Executor, query::{Pagination, Statement}};

pub struct SQLite {
    db: Pool<sqlx::sqlite::Sqlite>,
}

impl SQLite {
    pub async fn connect(url: &str) -> Result<Self> {
        return Ok(Self {
            db: SqlitePool::connect(url).await.unwrap()
        });
    }
}

impl Executor for SQLite {
    async fn get<O>(&self, statement: Statement) -> Result<Vec<O>>
    where
        O: for<'r> FromRow<'r,  <sqlx::Any as sqlx::Database>::Row> + Send + Unpin
    {
        todo!()
    }

    async fn pagination<O>(&self, statement: Statement) -> Result<Pagination<O>>
    where
        O: for<'r> FromRow<'r,  <sqlx::Any as sqlx::Database>::Row> + Send + Unpin
    {
        todo!()
    }
}