use sqlx::{ColumnIndex, Database, FromRow, IntoArguments, Pool};
use crate::{Entity, Executor, Result, types::Bindable};

pub struct Scalar<'q, E: Executor> 
where
    E::DB: Database,
{
    sql: Result<&'q str>,
    arguments: <E::DB as Database>::Arguments<'q>,
    executor: &'q E,
}

impl<'q, E: Executor> Scalar<'q, E> 
where
    E::DB: Database,
{
    pub fn new(executor: &'q E, sql: Result<&'q str>) -> Self {
        Self {
            sql,
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

impl<'q, E> Scalar<'q, E>
where
    E: Executor,
    E::DB: Database,
    usize: ColumnIndex<<E::DB as Database>::Row>,
    <E::DB as Database>::Arguments<'q>: IntoArguments<'q, E::DB>,
    for<'c> &'c Pool<E::DB>: sqlx::Executor<'c, Database = E::DB>,
{
    pub async fn first<O>(self) -> Result<O>
    where
        O: Send + Unpin,
        O: sqlx::Type<E::DB> + for<'r> sqlx::Decode<'r, E::DB>,
    {
        sqlx::query_scalar_with::<E::DB, O, _>(self.sql?, self.arguments)
            .fetch_one(self.executor.pool())
            .await
            .map_err(Into::into)
    }

    pub async fn first_optional<O>(self) -> Result<Option<O>>
    where
        O: Send + Unpin,
        O: sqlx::Type<E::DB> + for<'r> sqlx::Decode<'r, E::DB>,
        (O,): for<'r> FromRow<'r, <E::DB as Database>::Row>,
        O: Entity + for<'r> FromRow<'r, <E::DB as Database>::Row> + Send + Unpin,
    {
        sqlx::query_scalar_with::<E::DB, O, _>(self.sql?, self.arguments)
            .fetch_optional(self.executor.pool())
            .await
            .map_err(Into::into)
    }

    pub async fn all<O>(self) -> Result<Vec<O>>
    where
        O: Send + Unpin,
        O: sqlx::Type<E::DB> + for<'r> sqlx::Decode<'r, E::DB>,
        (O,): for<'r> FromRow<'r, <E::DB as Database>::Row>,
    {
        sqlx::query_scalar_with::<E::DB, O, _>(self.sql?, self.arguments)
            .fetch_all(self.executor.pool())
            .await
            .map_err(Into::into)
    }
}