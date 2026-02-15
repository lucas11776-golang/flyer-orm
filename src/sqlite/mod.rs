mod builder;

use anyhow::Result;
use sqlx::{FromRow, Pool, SqlitePool};

use crate::{Executor, query::{Pagination, Statement}};


#[derive(Debug)]
pub struct SQLite {
    // db: Pool<sqlx::sqlite::Sqlite>,
}

// impl SQLite {
//     pub async fn connect(url: &str) -> Result<Self> {
//         return Ok(Self {
//             db: SqlitePool::connect(url).await.unwrap()
//         });
//     }
// }


impl Executor for SQLite {
    type T = sqlx::Sqlite;


    async fn connect(url: &str) -> Self {
        return Self {

        }
    }

    fn table(&mut self, name: &str) -> &mut Self {
        todo!()
    }

    fn select(&mut self, columns: Vec<&str>) -> &mut Self {
        todo!()
    }

    fn order_by(&mut self, column: &str, order: crate::query::Order) -> &mut Self {
        todo!()
    }

    fn limit(&mut self, limit: u64) -> &mut Self {
        todo!()
    }

    async fn first<O>(&self) -> Result<Vec<O>>
    where
        O: for<'r> FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        todo!()
    }

    async fn get<O>(&self, limit: u64) -> Result<Vec<O>>
    where
        O: for<'r> FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        todo!()
    }

    fn pagination<O>(&self, limit: u64, page: u64) -> Result<Pagination<O>>
    where
        O: for<'r> FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
            todo!()
    }
}
