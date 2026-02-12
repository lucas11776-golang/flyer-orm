mod builder;

use anyhow::Result;
use sqlx::{FromRow, PgPool, Pool, Postgres as DBPostgres};

use crate::{Executor, query::{Pagination, Statement}};

pub struct Postgres {
    db: Pool<DBPostgres>,
}

impl Postgres {
    pub async fn connect(url: &str) -> Result<Self> {
        return Ok(Self {
            db: PgPool::connect(url).await.unwrap()
        });
    }
}

impl Executor for Postgres {
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