

use anyhow::Result;
use sqlx::{FromRow, MySql, MySqlPool, Pool};

use crate::{Executor, databases::mysql::query::MySQLQueryResult, query::{Pagination, Statement}};

mod builder;
pub mod query;

pub struct MySQL {
    _db: Pool<MySql>,
}

impl MySQL {
    pub async fn connect(url: &str) -> Result<Self> {
        return Ok(Self {
            _db: MySqlPool::connect(url).await.unwrap()
        });
    }
}

impl Executor for MySQL {
    type T = MySql;

    async fn new(_url: &str) -> Self where Self: Sized {
        todo!()
    }

    fn db<'q>(&'q self) -> &'q Pool<Self::T> {
        todo!()
    }

    fn to_sql<'q>(&self, _statement: &'q Statement<'q, Self::T>) -> String {
        todo!()
    }

    #[allow(refining_impl_trait)]
    async fn execute<'q>(&self, _sql: String, _args: <Self::T as sqlx::Database>::Arguments<'q>) -> Result<MySQLQueryResult> {
        todo!();
    }

    async fn execute_as<'q, O>(&self, _sql: String, _args: <Self::T as sqlx::Database>::Arguments<'q>) -> Result<Vec<O>>
    where
        O: for<'r> sqlx::prelude::FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        todo!()
    }
    
    async fn insert<'q>(&self, _statement: &'q Statement<'q, Self::T>) -> Result<()> {
        todo!()
    }
    
    async fn insert_as<'q, O>(&self, _statement: &'q Statement<'q, Self::T>) -> Result<O>
    where
        O: for<'r> sqlx::FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        todo!()
    }
    
    async fn update<'q>(&self, _statement: &'q Statement<'q, Self::T>) -> Result<()> {
        todo!()
    }
    
    async fn count<'q>(&self, _statement: &'q Statement<'q, Self::T>) -> Result<u64> {
        return Ok(0);
    }
    
    async fn delete<'q>(&self, _statement: &'q Statement<'q, Self::T>) -> Result<()> {
        todo!()
    }

    async fn query_all<'q, O, T: 'q + sqlx::Encode<'q, Self::T> + sqlx::Type<Self::T>>(&self, _sql: &str, _args: Vec<T>) -> Result<Vec<O>>
    where
        O: for<'r> FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized {
        todo!()
    }

    async fn query_one<'q, O, T: 'q + sqlx::Encode<'q, Self::T> + sqlx::Type<Self::T>>(&self, _sql: &str, _args: Vec<T>) -> Result<O>
    where
        O: for<'r> FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized {
        todo!()
    }

    async fn all<'q, O>(&self, _statement: &'q Statement<'q, Self::T>) -> Result<Vec<O>>
    where
        O: for<'r> FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized {
        todo!()
    }

    async fn first<'q, O>(&self, _statement: &'q Statement<'q, Self::T>) -> Result<O>
    where
        O: for<'r> FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized {
        todo!()
    }

    async fn paginate<'q, O>(&self, _statement: &'q Statement<'q, Self::T>) -> Result<Pagination<O>>
    where
        O: for<'r> FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized {
        todo!()
    }
}