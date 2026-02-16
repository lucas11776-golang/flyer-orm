mod builder;

use anyhow::Result;
use sqlx::Pool;

use crate::{Executor, QueryBuilder, query::Statement, sqlite::builder::Builder};

#[derive(Default)]
pub struct SQLite;

impl Executor for SQLite {
    type T = sqlx::Sqlite;

    async fn db(&self, url: &str) -> Result<Pool<Self::T>> {
        return Ok(sqlx::SqlitePool::connect(url).await.unwrap());
    }
    
    async fn get<'q, O>(&self, statement: &Statement<'q, Self::T>) -> Result<Vec<O>>
    where
        O: for<'r> sqlx::FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        return Ok(
            sqlx::query_as::<Self::T, O>(Builder::build(statement).unwrap().as_str())
                .fetch_all(&self.db(&statement.url).await.unwrap())
                .await
                .unwrap()
        )
    }
    
    async fn paginate<'q, O>(&self, _statement: &Statement<'q, Self::T>) -> Result<crate::query::Pagination<O>>
    where
        O: for<'r> sqlx::FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        todo!()
    }
}

