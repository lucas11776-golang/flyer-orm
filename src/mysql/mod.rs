mod builder;

use anyhow::Result;
use sqlx::{FromRow, MySql, MySqlPool, Pool};

use crate::{Executor, query::{Pagination, Statement}};

pub struct MySQL {
    db: Pool<MySql>,
}

impl MySQL {
    pub async fn connect(url: &str) -> Result<Self> {
        return Ok(Self {
            db: MySqlPool::connect(url).await.unwrap()
        });
    }
}

impl Executor for MySQL {
    type T = MySql;

    async fn new(url: &str) -> Self where Self: Sized {
        todo!()
    }

    fn db<'q>(&'q self) -> &'q Pool<Self::T> {
        todo!()
    }

    fn to_sql<'q>(&self, statement: &'q Statement<'q, Self::T>) -> Result<String> {
        todo!()
    }
    
    async fn insert<'q>(&self, statement: &'q Statement<'q, Self::T>) -> Result<bool> {
        todo!()
    }
    
    async fn insert_as<'q, O>(&self, statement: &'q Statement<'q, Self::T>) -> Result<O>
    where
        O: for<'r> sqlx::FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        todo!()
    }

    async fn query_all<'q, O, T: 'q + sqlx::Encode<'q, Self::T> + sqlx::Type<Self::T>>(&self, sql: &str, args: Vec<T>) -> Result<Vec<O>>
    where
        O: for<'r> FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized {
        todo!()
    }

    async fn query_one<'q, O, T: 'q + sqlx::Encode<'q, Self::T> + sqlx::Type<Self::T>>(&self, sql: &str, args: Vec<T>) -> Result<O>
    where
        O: for<'r> FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized {
        todo!()
    }

    async fn all<'q, O>(&self, statement: &'q Statement<'q, Self::T>) -> Result<Vec<O>>
    where
        O: for<'r> FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized {
        todo!()
    }

    async fn first<'q, O>(&self, statement: &'q Statement<'q, Self::T>) -> Result<O>
    where
        O: for<'r> FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized {
        todo!()
    }

    async fn paginate<'q, O>(&self, statement: &'q Statement<'q, Self::T>) -> Result<Pagination<O>>
    where
        O: for<'r> FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized {
        todo!()
    }
}