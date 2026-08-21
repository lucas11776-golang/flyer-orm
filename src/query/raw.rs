use std::mem;

use sqlx::IntoArguments;

use crate::{Entity, Executor, Result, types::Bindable};

pub struct Raw<'e, E: Executor> {
    sql: Result<&'e str>,
    arguments: <E::DB as sqlx::Database>::Arguments<'e>,
    executor: &'e E,
}

impl <'e, E: Executor>Raw<'e, E> {
    pub fn new(executor: &'e E, sql: Result<&'e str>) -> Self {
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

    pub async fn execute(mut self) -> Result<()> {
        self
            .executor
            .execute(String::from(self.sql?), mem::take(&mut self.arguments))
            .await
            .map(|_| {})
    }

    pub async fn first<O>(mut self) -> Result<O>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <E::DB as sqlx::Database>::Row> + Send + Unpin,
        for<'a> <E::DB as sqlx::Database>::Arguments<'a>: IntoArguments<'a, E::DB>,
        for<'c> &'c mut <E::DB as sqlx::Database>::Connection: sqlx::Executor<'c, Database = E::DB>,
    {
        self.executor
            .fetch_one(self.sql?, mem::take(&mut self.arguments))
            .await
    }

    pub async fn all<O>(mut self) -> Result<Vec<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <E::DB as sqlx::Database>::Row> + Send + Unpin,
        for<'a> <E::DB as sqlx::Database>::Arguments<'a>: IntoArguments<'a, E::DB>,
        for<'c> &'c mut <E::DB as sqlx::Database>::Connection: sqlx::Executor<'c, Database = E::DB>,
    {
        self
            .executor
            .fetch_all::<O>(self.sql?, mem::take(&mut self.arguments))
            .await
    }
}