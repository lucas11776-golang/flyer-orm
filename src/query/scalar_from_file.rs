use sqlx::{ColumnIndex, Database, IntoArguments};
use crate::{Executor, Result, types::Bindable};

pub struct ScalarFromFile<'q, E: Executor> 
where
    E::DB: Database,
{
    sql: &'q str,
    arguments: <E::DB as Database>::Arguments<'q>,
    executor: &'q E,
}

impl <'q, E: Executor>ScalarFromFile<'q, E> 
where
    E::DB: Database,
{
    pub fn new(executor: &'q E, sql: &'q str) -> Self {
        Self {
            sql: sql,
            arguments: Default::default(),
            executor,
        }
    }

    pub fn bind<V>(mut self, value: V) -> Self
    where
        V: Bindable<E::DB>,
    {
        value
            .bind_to(&mut self.arguments)
            .unwrap();
        self
    }
}

impl <'q, E: Executor + 'static>ScalarFromFile<'q, E>
where
{
    pub async fn first<O>(self) -> Result<O>
    where
        O: Send + Unpin,
        O: sqlx::Type<E::DB> + for<'r> sqlx::Decode<'r, E::DB>,
        usize: ColumnIndex<<E::DB as Database>::Row>,
        <E::DB as sqlx::Database>::Arguments<'q>: IntoArguments<'q, E::DB>,
        for<'c> &'c mut <E::DB as sqlx::Database>::Connection: sqlx::Executor<'c, Database = E::DB>,
    {
        self
            .executor
            .fetch_one_scalar(self.sql, self.arguments)
            .await
    }

    pub async fn first_optional<O>(self) -> Result<Option<O>>
    where
        O: Send + Unpin,
        O: sqlx::Type<E::DB> + for<'r> sqlx::Decode<'r, E::DB>,
        usize: ColumnIndex<<E::DB as Database>::Row>,
        <E::DB as sqlx::Database>::Arguments<'q>: IntoArguments<'q, E::DB>,
        for<'c> &'c mut <E::DB as sqlx::Database>::Connection: sqlx::Executor<'c, Database = E::DB>,
    {
        self
            .executor
            .fetch_one_scalar(self.sql, self.arguments)
            .await
    }

    pub async fn all<O>(self) -> Result<Vec<O>>
    where
        O: Send + Unpin,
        O: sqlx::Type<E::DB> + for<'r> sqlx::Decode<'r, E::DB>,
        usize: ColumnIndex<<E::DB as Database>::Row>,
        <E::DB as sqlx::Database>::Arguments<'q>: IntoArguments<'q, E::DB>,
        for<'c> &'c mut <E::DB as sqlx::Database>::Connection: sqlx::Executor<'c, Database = E::DB>,
    {
        self
            .executor
            .fetch_all_scalar(self.sql, self.arguments)
            .await
    }
}