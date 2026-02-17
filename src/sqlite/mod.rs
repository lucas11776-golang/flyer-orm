mod builder;


use anyhow::Result;
use sqlx::Pool;

use crate::{Executor, QueryBuilder, query::{Pagination, Statement, Total}, sqlite::builder::Builder};

#[derive(Default)]
pub struct SQLite;

impl Executor for SQLite {
    type T = sqlx::Sqlite;
    
    async fn db<'q>(&self, url: &str) -> Result<Pool<Self::T>> {
        return Ok(sqlx::SqlitePool::connect(url).await.unwrap());
    }
    
    fn to_sql<'q>(&'q self, statement: &'q Statement<'q, Self::T>) -> Result<String> {
        return Ok(Builder::build(&statement.query).unwrap());
    }
    
    async fn first<'q, O>(&self, statement: &'q Statement<'q, Self::T>) -> Result<O>
    where
        O: for<'r> sqlx::FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        return Ok(
            sqlx::query_as::<Self::T, O>(&Builder::build(&statement.query).unwrap())
                .fetch_one(&self.db(&statement.url).await.unwrap())
                .await
                .unwrap()
        );
    }
    
    async fn get<'q, O>(&self, statement: &'q Statement<'q, Self::T>) -> Result<Vec<O>>
    where
        O: for<'r> sqlx::FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        return Ok(
            sqlx::query_as::<Self::T, O>(&Builder::build(&statement.query).unwrap())
                .fetch_all(&self.db(&statement.url).await.unwrap())
                .await
                .unwrap(),
        );
    }
    
    async fn paginate<'q, O>(&self, statement: &'q Statement<'q, Self::T>) -> Result<Pagination<O>>
    where
        O: for<'r> sqlx::FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        let mut query = statement.query.clone();

        query.select = vec!["COUNT(*) as total".to_string()];
        query.limit = None;
        query.page = None;

        let db = self.db(&statement.url).await.unwrap();

        let total = sqlx::query_as::<Self::T, Total>(&Builder::build(&query).unwrap())
            .fetch_one(&db)
            .await
            .unwrap();

        let items = sqlx::query_as::<Self::T, O>(&Builder::build(&statement.query).unwrap())
            .fetch_all(&self.db(&statement.url).await.unwrap())
            .await
            .unwrap();

        return Ok(
            Pagination {
                total: total.total,
                page: statement.query.page.unwrap(),
                per_page: statement.query.limit.unwrap(),
                items: items,
            }
        );
    }
}

