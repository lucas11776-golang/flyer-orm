use std::mem;

use crate::{Database, Entity, Executor, Result, query::FromFile, types::Bindable};

pub struct RawFromFile<'e, E: Executor + 'static> {
    path: String,
    arguments: <E::DB as sqlx::Database>::Arguments<'e>,
    executor: &'e E,
}

impl <'e, E: Executor>FromFile<E> for RawFromFile<'e, E> { }

impl <'e, E: Executor>RawFromFile<'e, E> {
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

    async fn sql(&self) -> Result<String> {
        self
            .read(self.path.clone())
            .await
    }

    pub async fn execute(self) -> Result<()> {
        self
            .executor
            .execute(self.sql().await?, self.arguments)
            .await
            .map(|_| {})
    }

    pub async fn first<O>(self) -> Result<O>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <E::DB as sqlx::Database>::Row> + Send + Unpin, 
    {
        self
            .executor
            .fetch_one(self.sql().await?, self.arguments)
            .await
    }

    pub async fn all<O>(self) -> Result<Vec<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <E::DB as sqlx::Database>::Row> + Send + Unpin, 
    {
        self
            .executor
            .fetch_all::<O>(self.sql().await?, self.arguments)
            .await
    }
}