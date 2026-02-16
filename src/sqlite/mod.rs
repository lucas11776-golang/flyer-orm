mod builder;

use anyhow::Result;
use sqlx::Pool;

use crate::{Executor, QueryBuilder, query::{Pagination, Statement, Total}, sqlite::builder::Builder};

#[derive(Default)]
pub struct SQLite;

impl Executor for SQLite {
    type T = sqlx::Sqlite;

    async fn db(&self, url: &str) -> Result<Pool<Self::T>> {
        return Ok(sqlx::SqlitePool::connect(url).await.unwrap());
    }
    
    async fn first<'q, O>(&self, statement: &mut Statement<'q, Self::T>) -> Result<O>
    where
        O: for<'r> sqlx::FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        return Ok(
            sqlx::query_as::<Self::T, O>(Builder::build(statement).unwrap().as_str())
                .fetch_one(&self.db(&statement.url).await.unwrap())
                .await
                .unwrap()
        )
    }
    
    async fn get<'q, O>(&self, statement: &mut Statement<'q, Self::T>) -> Result<Vec<O>>
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
    
    async fn paginate<'q, O>(&self, statement: &mut Statement<'q, Self::T>) -> Result<crate::query::Pagination<O>>
    where
        O: for<'r> sqlx::FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        let select_store = statement.select.clone();

        statement.select = vec!["COUNT(*) as total".to_string()];

        let total: Total = self.first::<Total>(statement).await.unwrap();

        statement.select = select_store;

        return Ok(Pagination {
            total: total.total,
            page: statement.page.unwrap(),
            per_page: statement.limit.unwrap(),
            items: self.get(statement).await.unwrap(),
        })
    }
}

