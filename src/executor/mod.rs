use sqlx::{Database as SqlxDatabase, Pool};

use crate::Entity;
use crate::Result;

use crate::{
    query::{Pagination, Statement},
    types::QueryResult,
};

#[allow(async_fn_in_trait)]
pub trait Executor: Send + Sync {
    type DB: SqlxDatabase;

    async fn new(url: impl Into<String>) -> Self;

    fn from(pool: Pool<Self::DB>) -> Self;

    fn to_sql<'a>(&self, statement: &'a Statement<Self::DB>) -> String;

    fn db(&self) -> &Pool<Self::DB>;

    async fn execute<'a>(&'a self, sql: String, arguments: <Self::DB as SqlxDatabase>::Arguments<'a>) -> Result<impl QueryResult>;

    async fn fetch_one<'a, O>(&'a self, sql: String, arguments: <Self::DB as SqlxDatabase>::Arguments<'a>) -> Result<O>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as SqlxDatabase>::Row> + Send + Unpin;

    async fn fetch_all<'a, O>(&'a self, sql: String, arguments: <Self::DB as SqlxDatabase>::Arguments<'a>) -> Result<Vec<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as SqlxDatabase>::Row> + Send + Unpin;

    async fn insert<'a>(&'a self, statement: &'a Statement<Self::DB>) -> Result<impl QueryResult>;

    async fn update<'a>(&'a self, statement: &'a Statement<Self::DB>) -> Result<impl QueryResult>;

    async fn count<'a>(&'a self, statement: &'a Statement<Self::DB>) -> Result<i64>;

    async fn delete<'a>(&'a self, statement: &'a Statement<Self::DB>) -> Result<impl QueryResult>;

    async fn insert_as<'a, O>(&'a self, statement: &'a Statement<Self::DB>) -> Result<O>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as SqlxDatabase>::Row> + Send + Unpin;

    async fn all<'a, O>(&'a self, statement: &'a Statement<Self::DB>) -> Result<Vec<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as SqlxDatabase>::Row> + Send + Unpin;

    async fn first<'a, O>(&'a self, statement: &'a Statement<Self::DB>) -> Result<O>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as SqlxDatabase>::Row> + Send + Unpin;

    async fn paginate<'a, O>(&'a self, statement: &'a Statement<Self::DB>) -> Result<Pagination<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as SqlxDatabase>::Row> + Send + Unpin;
}