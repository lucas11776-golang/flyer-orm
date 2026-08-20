use sqlx::{ColumnIndex, Database, FromRow, IntoArguments, Pool};
use crate::{Entity, Executor, Result, query::FromFile, types::Bindable};

pub struct ScalarFromFile<'q, E: Executor> 
where
    E::DB: Database,
{
    path: String,
    arguments: <E::DB as Database>::Arguments<'q>,
    executor: &'q E,
}

impl <'q, E: Executor + 'static>FromFile<E> for ScalarFromFile<'q, E> {} 

impl <'q, E: Executor>ScalarFromFile<'q, E> 
where
    E::DB: Database,
{
    pub fn new(executor: &'q E, path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
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

// TODO: move scaler function to Executor.
impl <'q, E: Executor + 'static>ScalarFromFile<'q, E>
where
    E::DB: Database,
    usize: ColumnIndex<<E::DB as Database>::Row>,
    <E::DB as Database>::Arguments<'q>: IntoArguments<'q, E::DB>,
    for<'c> &'c Pool<E::DB>: sqlx::Executor<'c, Database = E::DB>,
{
    async fn sql(&self) -> Result<String> {
        self
            .read(self.path.clone())
            .await
    }

    pub async fn first<O>(self) -> Result<O>
    where
        O: Send + Unpin,
        O: sqlx::Type<E::DB> + for<'r> sqlx::Decode<'r, E::DB>,
    {
        sqlx::query_scalar_with::<E::DB, O, _>(self.sql().await?, self.arguments)
            .fetch_one(self.executor.pool())
            .await
            .map_err(Into::into)
    }

    pub async fn first_optional<O>(self) -> Result<Option<O>>
    where
        O: Send + Unpin,
        O: sqlx::Type<E::DB> + for<'r> sqlx::Decode<'r, E::DB>,
        (O,): for<'r> FromRow<'r, <E::DB as Database>::Row>,
    {
        sqlx::query_scalar_with::<E::DB, O, _>(self.sql().await?, self.arguments)
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
        sqlx::query_scalar_with::<E::DB, O, _>(self.sql().await?, self.arguments)
            .fetch_all(self.executor.pool())
            .await
            .map_err(Into::into)
    }
}