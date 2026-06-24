use std::marker::PhantomData;

use anyhow::Result;
use sqlx::{Arguments, Encode, FromRow, types::Type};

use crate::{executor::Executor, query::QueryResult,};

pub struct QueryScaler<'q, E: Executor> {
    executor: &'q E,
    pub(crate) sql: String,
    arguments: <E::T as sqlx::Database>::Arguments<'q>,
    _marker: PhantomData<E>
}

impl <'q, E>QueryScaler<'q, E>
where
    E: Executor
{
    pub fn new(exc: &'q E, sql: String) -> Self {
        return Self {
            executor: exc,
            sql: sql,
            arguments: Default::default(),
            _marker: PhantomData
        }
    }

    pub fn bind<T: 'q + Encode<'q, E::T> + Type<E::T>>(mut self, value: T) -> Self {
        self.arguments.add(value).unwrap();

        return self;
    }

    // TODO: need to impl `query_scalar` in `Executor`.
    pub async fn execute(self) -> Result<impl QueryResult> {
        return self.executor
            .execute(self.sql.clone(), self.arguments)
            .await;
    }

    pub async fn execute_as<O>(self) -> Result<Vec<O>>
    where
        O: for<'r> FromRow<'r, <E::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        return self.executor.query_scalar_with(self.sql, self.arguments).await;
    }
}