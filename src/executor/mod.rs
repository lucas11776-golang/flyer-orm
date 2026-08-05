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

    fn to_sql<'q>(&self, statement: &Statement<Self::DB>) -> String;

    fn db(&self) -> &Pool<Self::DB>;

    async fn execute<'c>(&self, sql: String, arguments: <Self::DB as SqlxDatabase>::Arguments<'c>) -> Result<impl QueryResult>;

    async fn fetch_one<'c, O>(&self, sql: String, arguments: <Self::DB as SqlxDatabase>::Arguments<'c>) -> Result<O>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as SqlxDatabase>::Row> + Send + Unpin;

    async fn fetch_all<'c, O>(&self, sql: String, arguments: <Self::DB as SqlxDatabase>::Arguments<'c>) -> Result<Vec<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as SqlxDatabase>::Row> + Send + Unpin;

    async fn insert<'q>(&self, statement: &Statement<Self::DB>) -> Result<impl QueryResult>;

    async fn update<'q>(&self, statement: &Statement<Self::DB>) -> Result<impl QueryResult>;

    async fn count<'q>(&self, statement: &Statement<Self::DB>) -> Result<i64>;

    async fn delete<'q>(&self, statement: &Statement<Self::DB>) -> Result<impl QueryResult>;

    async fn insert_as<'q, O>(&self, statement: &Statement<Self::DB>) -> Result<O>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as SqlxDatabase>::Row> + Send + Unpin;

    async fn all<O>(&self, statement: &Statement<Self::DB>) -> Result<Vec<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as SqlxDatabase>::Row> + Send + Unpin;

    async fn first<O>(&self, statement: &Statement<Self::DB>) -> Result<O>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as SqlxDatabase>::Row> + Send + Unpin;

    async fn paginate<O>(&self, statement: &Statement<Self::DB>) -> Result<Pagination<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as SqlxDatabase>::Row> + Send + Unpin;
}