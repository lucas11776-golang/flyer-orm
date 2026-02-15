mod builder;

use anyhow::Result;
use sqlx::Pool;

use crate::{Executor, query::{Statement}};

#[derive(Default)]
pub struct SQLite;

impl Executor for SQLite {
    type T = sqlx::Sqlite;

    async fn db(&self, url: &str) -> Result<Pool<Self::T>> {
        return Ok(sqlx::SqlitePool::connect(url).await.unwrap());
    }
    
    async fn get<O>(&self, statement: &Statement) -> Result<Vec<O>>
    where
        O: for<'r> sqlx::FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        return Ok(
            sqlx::query_as::<Self::T, O>(format!("SELECT * FROM `{}`", statement.table).as_str()) // TODO: Impl QueryBuilder
                .fetch_all(&self.db(&statement.url).await.unwrap())
                .await
                .unwrap()
        )
    }
    
    async fn paginate<O>(&self, _statement: &Statement) -> Result<crate::query::Pagination<O>>
    where
        O: for<'r> sqlx::FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized
        {
        todo!()
    }
}

