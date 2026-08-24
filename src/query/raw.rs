use std::sync::Arc;

use sqlx::IntoArguments;

use crate::{Entity, Executor, Result, types::Bindable, utils::args_as_ref};

pub struct Raw<E: Executor> {
    sql: Result<String>,
    arguments: Vec<Box<dyn Bindable<E::DB>>>,
    executor: Arc<E>,
}

impl <E: Executor>Raw<E> {
    pub fn new(executor: Arc<E>, sql: Result<String>) -> Self {
        Self {
            sql: sql,
            arguments: Default::default(),
            executor: executor,
        }
    }

    #[inline]
    pub fn bind<V>(mut self, value: V) -> Self
    where
        V: Bindable<E::DB>,
    {
        self
            .arguments
            .push(Box::new(value));
        self
    }
}

impl <E: Executor>Raw<E>
where
    for<'a> <E::DB as sqlx::Database>::Arguments<'a>: IntoArguments<'a, E::DB>,
    for<'c> &'c mut <E::DB as sqlx::Database>::Connection: sqlx::Executor<'c, Database = E::DB>,
{
    pub async fn execute(self) -> Result<()> {
        self
            .executor
            .execute(String::from(self.sql?), Default::default())
            .await
            .map(|_| ())
    }

    pub async fn first<O>(self) -> Result<O>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <E::DB as sqlx::Database>::Row> + Send + Unpin,
    {
        self
            .executor
            .fetch_one(self.sql?, args_as_ref(&self.arguments))
            .await
    }

    pub async fn all<O>(self) -> Result<Vec<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <E::DB as sqlx::Database>::Row> + Send + Unpin,
    {
        self
            .executor
            .fetch_all::<O>(self.sql?, args_as_ref(&self.arguments))
            .await
    }
}