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
    async fn get<T: sqlx::Database, O>(&self, statement: Statement) -> Result<Vec<O>>
    where
        O: for<'r> FromRow<'r,  <T as sqlx::Database>::Row> + Send + Unpin
    {
        // Ok(
        //     sqlx::query_as::<_, O>(format!("SELECT * FROM `{}`", statement.table))
        //         .fetch_all(&self.db)
        //         .await
        //         .unwrap()
        // );


        todo!();
    }

    async fn pagination<T: sqlx::Database, O>(&self, statement: Statement) -> Result<Pagination<O>>
    where
        O: for<'r> FromRow<'r,  <T as sqlx::Database>::Row> + Send + Unpin
    {
        todo!()
    }
}