use std::mem;

use crate::{Entity, Executor, Result, types::{Bindable, QueryResult}};

pub struct Raw<'e, E: Executor> {
    sql: String,
    arguments: <E::DB as sqlx::Database>::Arguments<'e>,
    executor: &'e E,
}

impl <'e, E: Executor>Raw<'e, E> {
    pub fn new(executor: &'e E, sql: impl Into<String>) -> Self {
        return Self {
            sql: sql.into(),
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

    pub async fn execute(mut self) -> Result<impl QueryResult> {
        self
            .executor
            .execute(self.sql.clone(), mem::take(&mut self.arguments))
            .await
    }

    pub async fn first<O>(mut self) -> Result<O>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <E::DB as sqlx::Database>::Row> + Send + Unpin, 
    {
        self
            .executor
            .fetch_one(self.sql.clone(), mem::take(&mut self.arguments))
            .await
    }

    pub async fn all<O>(mut self) -> Result<Vec<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <E::DB as sqlx::Database>::Row> + Send + Unpin, 
    {
        self
            .executor
            .fetch_all(self.sql.clone(), mem::take(&mut self.arguments))
            .await
    }
}