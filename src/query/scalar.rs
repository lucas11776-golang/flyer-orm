use std::sync::Arc;

use sqlx::{ColumnIndex, Database, IntoArguments, Pool};

use crate::{types::Bindable, Executor, Result};

pub struct Scalar<E: Executor>
where
    E::DB: Database,
{
    sql: Result<String>,
    arguments: Vec<Box<dyn Bindable<E::DB>>>,
    executor: Arc<E>,
}

impl<E: Executor> Scalar<E>
where
    E::DB: Database,
{
    #[inline]
    pub fn new(executor: Arc<E>, sql: Result<String>) -> Self {
        Self {
            sql: sql,
            arguments: Default::default(),
            executor: executor,
        }
    }

    pub fn bind<V>(mut self, value: V) -> Self
    where
        V: Bindable<E::DB>,
    {
        self.arguments.push(Box::new(value));
        self
    }
}

impl <E>Scalar<E>
where
    E: Executor,
    E::DB: Database,
    usize: ColumnIndex<<E::DB as Database>::Row>,
    for <'q> <E::DB as Database>::Arguments<'q>: IntoArguments<'q, E::DB>,
    for<'c> &'c Pool<E::DB>: sqlx::Executor<'c, Database = E::DB>,
{
    fn get_arguments<'a>(args: Vec<Box<dyn Bindable<E::DB>>>) -> <E::DB as sqlx::Database>::Arguments<'a> {
        let mut arguments= Default::default();

        for arg in args {
            arg.bind_to(&mut arguments).unwrap();
        }

        return arguments;
    }

    pub async fn first<O>(self) -> Result<O>
    where
        O: Send + Unpin + sqlx::Type<E::DB> + for<'r> sqlx::Decode<'r, E::DB>,
    {
        sqlx::query_scalar_with::<E::DB, O, _>(&self.sql?, Self::get_arguments(self.arguments))
            .fetch_one(self.executor.pool())
            .await
            .map_err(Into::into)
    }

    pub async fn first_optional<O>(self) -> Result<Option<O>>
    where
        O: Send + Unpin + sqlx::Type<E::DB> + for<'r> sqlx::Decode<'r, E::DB>,
    {
        sqlx::query_scalar_with::<E::DB, O, _>(&self.sql?, Self::get_arguments(self.arguments))
            .fetch_optional(self.executor.pool())
            .await
            .map_err(Into::into)
    }

    pub async fn all<O>(self) -> Result<Vec<O>>
    where
        O: Send + Unpin + sqlx::Type<E::DB> + for<'r> sqlx::Decode<'r, E::DB>,
    {
        sqlx::query_scalar_with::<E::DB, O, _>(&self.sql?, Self::get_arguments(self.arguments))
            .fetch_all(self.executor.pool())
            .await
            .map_err(Into::into)
    }
}