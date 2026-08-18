use std::mem;

use crate::{Database, Entity, Executor, Result, types::Bindable};

pub struct RawRead<'e, E: Executor + 'static> {
    path: String,
    arguments: <E::DB as sqlx::Database>::Arguments<'e>,
    executor: &'e E,
}

// TODO: fix handle read error.
impl <'e, E: Executor>RawRead<'e, E> {
    pub fn new(executor: &'e E, path: impl Into<String>) -> Self {
        return Self {
            path: path.into(),
            arguments: Default::default(),
            executor: executor,
        };
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

    async fn read(&self) -> Result<String> {
        Database::<E>::cache(self.path.clone()).await
    }

    pub async fn execute(mut self) -> Result<()> {
        self
            .executor
            .execute(self.read().await.unwrap(), mem::take(&mut self.arguments))
            .await
            .map(|_| {})
    }

    pub async fn first<O>(mut self) -> Result<O>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <E::DB as sqlx::Database>::Row> + Send + Unpin, 
    {
        self
            .executor
            .fetch_one(self.read().await.unwrap(), mem::take(&mut self.arguments))
            .await
    }

    pub async fn all<O>(mut self) -> Result<Vec<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <E::DB as sqlx::Database>::Row> + Send + Unpin, 
    {
        self
            .executor
            .fetch_all::<O>(self.read().await.unwrap(), mem::take(&mut self.arguments))
            .await
    }
}