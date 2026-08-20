use sqlx::{ColumnIndex, IntoArguments};
use sqlx::{Database, Pool};

use crate::Entity;
use crate::Result;

use crate::{
    query::{Pagination, Statement},
    types::QueryResult,
};

#[allow(async_fn_in_trait)]
pub trait Executor: Send + Sync {
    type DB: Database;

    async fn new(url: impl Into<String>) -> Self;

    fn from(pool: Pool<Self::DB>) -> Self;

    fn to_sql<'a>(&self, statement: &'a Statement<Self::DB>) -> String;

    fn pool(&self) -> &Pool<Self::DB>;

    async fn execute<'a>(&'a self, sql: String, arguments: <Self::DB as Database>::Arguments<'a>) -> Result<impl QueryResult>;

    async fn fetch_one<'a, O>(
        &'a self,
        sql: &'a str,
        arguments: <Self::DB as sqlx::Database>::Arguments<'a>,
    ) -> Result<O>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin,
        <Self::DB as sqlx::Database>::Arguments<'a>: IntoArguments<'a, Self::DB>,
        for<'c> &'c mut <Self::DB as sqlx::Database>::Connection: sqlx::Executor<'c, Database = Self::DB>,
    {
        sqlx::query_as_with::<Self::DB, O, _>(sql, arguments)
            .fetch_one(self.pool())
            .await
            .map_err(Into::into)
    }

    async fn fetch_all<'a, O>(
        &'a self,
        sql: &'a str,
        arguments: <Self::DB as sqlx::Database>::Arguments<'a>,
    ) -> Result<Vec<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin,
        <Self::DB as sqlx::Database>::Arguments<'a>: IntoArguments<'a, Self::DB>,
        for<'c> &'c mut <Self::DB as sqlx::Database>::Connection: sqlx::Executor<'c, Database = Self::DB>,
    {
        sqlx::query_as_with::<Self::DB, O, _>(&sql, arguments)
            .fetch_all(self.pool())
            .await
            .map_err(Into::into)
    }

    async fn fetch_one_scalar<'a, O>(&'a self, sql: &'a str, arguments: <Self::DB as sqlx::Database>::Arguments<'a>) -> Result<O>
    where
        O: Send + Unpin,
        O: sqlx::Type<Self::DB> + for<'r> sqlx::Decode<'r, Self::DB>,
        usize: ColumnIndex<<Self::DB as Database>::Row>,
        <Self::DB as sqlx::Database>::Arguments<'a>: IntoArguments<'a, Self::DB>,
        for<'c> &'c mut <Self::DB as sqlx::Database>::Connection: sqlx::Executor<'c, Database = Self::DB>,
    {
        sqlx::query_scalar_with::<Self::DB, O, _>(sql, arguments)
            .fetch_one(self.pool())
            .await
            .map_err(Into::into)
    }

    async fn fetch_optional_scalar<'a, O>(&'a self, sql: &'a str, arguments: <Self::DB as sqlx::Database>::Arguments<'a>) -> Result<Option<O>>
    where
        O: Send + Unpin,
        O: sqlx::Type<Self::DB> + for<'r> sqlx::Decode<'r, Self::DB>,
        usize: ColumnIndex<<Self::DB as Database>::Row>,
        <Self::DB as sqlx::Database>::Arguments<'a>: IntoArguments<'a, Self::DB>,
        for<'c> &'c mut <Self::DB as sqlx::Database>::Connection: sqlx::Executor<'c, Database = Self::DB>,
    {
        sqlx::query_scalar_with::<Self::DB, O, _>(sql, arguments)
            .fetch_optional(self.pool())
            .await
            .map_err(Into::into)
    }


    async fn fetch_all_scalar<'a, O>(&'a self, sql: &'a str, arguments: <Self::DB as sqlx::Database>::Arguments<'a>) -> Result<Vec<O>>
    where
        O: Send + Unpin,
        O: sqlx::Type<Self::DB> + for<'r> sqlx::Decode<'r, Self::DB>,
        usize: ColumnIndex<<Self::DB as Database>::Row>,
        <Self::DB as sqlx::Database>::Arguments<'a>: IntoArguments<'a, Self::DB>,
        for<'c> &'c mut <Self::DB as sqlx::Database>::Connection: sqlx::Executor<'c, Database = Self::DB>,
    {
        sqlx::query_scalar_with::<Self::DB, O, _>(sql, arguments)
            .fetch_all(self.pool())
            .await
            .map_err(Into::into)
    }

    async fn insert<'a>(&'a self, statement: &'a Statement<Self::DB>) -> Result<impl QueryResult>;

    async fn update<'a>(&'a self, statement: &'a Statement<Self::DB>) -> Result<impl QueryResult>;

    async fn count<'a>(&'a self, statement: &'a Statement<Self::DB>) -> Result<i64>;

    async fn delete<'a>(&'a self, statement: &'a Statement<Self::DB>) -> Result<impl QueryResult>;

    async fn insert_as<'a, O>(&'a self, statement: &'a Statement<Self::DB>) -> Result<O>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as Database>::Row> + Send + Unpin;

    async fn all<'a, O>(&'a self, statement: &'a Statement<Self::DB>) -> Result<Vec<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as Database>::Row> + Send + Unpin;

    async fn first<'a, O>(&'a self, statement: &'a Statement<Self::DB>) -> Result<O>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as Database>::Row> + Send + Unpin;

    async fn paginate<'a, O>(&'a self, statement: &'a Statement<Self::DB>) -> Result<Pagination<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as Database>::Row> + Send + Unpin;
}