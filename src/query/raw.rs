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

    pub async fn execute(self) -> Result<()> {
        self
            .executor
            .execute(String::from(self.sql?), self.arguments)
            .await
            .map(|_| {})
    }

    pub async fn first<O>(self) -> Result<O>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <E::DB as sqlx::Database>::Row> + Send + Unpin,
        for<'a> <E::DB as sqlx::Database>::Arguments<'a>: IntoArguments<'a, E::DB>,
        for<'c> &'c mut <E::DB as sqlx::Database>::Connection: sqlx::Executor<'c, Database = E::DB>,
    {
        self.executor
            .fetch_one(self.sql?, self.arguments)
            .await
    }

    pub async fn all<O>(self) -> Result<Vec<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <E::DB as sqlx::Database>::Row> + Send + Unpin,
        for<'a> <E::DB as sqlx::Database>::Arguments<'a>: IntoArguments<'a, E::DB>,
        for<'c> &'c mut <E::DB as sqlx::Database>::Connection: sqlx::Executor<'c, Database = E::DB>,
    {
        self
            .executor
            .fetch_all::<O>(self.sql?, self.arguments)
            .await
    }
}