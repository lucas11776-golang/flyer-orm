use anyhow::Result;
use sqlx::{Encode, FromRow, Pool, Type};

use crate::query::{Pagination, QueryResult, Statement};

#[allow(async_fn_in_trait)]
pub trait Executor {
    type T: sqlx::Database;

    async fn new(url: &str) -> Self where Self: Sized;

    fn db<'q>(&'q self) -> &'q Pool<Self::T>; 

    fn to_sql<'q>(&self, statement: &'q Statement<'q, Self::T>) -> String;

    async fn execute<'q>(&self, sql: String, args: <Self::T as sqlx::Database>::Arguments<'q>) -> Result<impl QueryResult>;

    async fn insert<'q>(&self, statement: &'q Statement<'q, Self::T>) -> Result<()>;

    async fn update<'q>(&self, statement: &'q Statement<'q, Self::T>) -> Result<()>;

    async fn count<'q>(&self, statement: &'q Statement<'q, Self::T>) -> Result<u64>;

    async fn delete<'q>(&self, statement: &'q Statement<'q, Self::T>) -> Result<()>;

    async fn insert_as<'q, O>(&self, statement: &'q Statement<'q, Self::T>) -> Result<O>
    where
        O: for<'r> FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized;

    async fn query_all<'q, O, T: 'q + Encode<'q, Self::T> + Type<Self::T>>(&self, sql: &str, args: Vec<T>) -> Result<Vec<O>>
    where
        O: for<'r> FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized;

    async fn query_one<'q, O, T: 'q + Encode<'q, Self::T> + Type<Self::T>>(&self, sql: &str, args: Vec<T>) -> Result<O>
    where
        O: for<'r> FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized;

    async fn all<'q, O>(&self, statement: &'q Statement<'q, Self::T>) -> Result<Vec<O>>
    where
        O: for<'r> FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized;

    async fn first<'q, O>(&self, statement: &'q Statement<'q, Self::T>) -> Result<O>
    where
        O: for<'r> FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized;

    async fn paginate<'q, O>(&self, statement: &'q Statement<'q, Self::T>) -> Result<Pagination<O>>
    where
        O: for<'r> FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized;
}