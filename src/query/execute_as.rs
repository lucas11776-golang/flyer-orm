use std::mem;

use crate::{Entity, Executor, Result, types::Bindable};

pub struct ExecuteAs<'e, E: Executor> {
    sql: String,
    arguments: <E::DB as sqlx::Database>::Arguments<'e>,
    executor: &'e E,
    
}

impl <'e, E: Executor>ExecuteAs<'e, E> {
    pub fn new(executor: &'e E, sql: impl Into<String>) -> Self {
        return Self {
            sql: sql.into(),
            arguments: Default::default(),
            executor: executor,
        };
    }

    pub fn bind<V, O>(&mut self, value: V) -> Result<Vec<O>>
    where
        V: Bindable<E::DB>,
        O: Entity + for<'r> sqlx::FromRow<'r, <E::DB as sqlx::Database>::Row> + Send + Unpin, 
    {
        todo!()
    }

    pub async fn fetch_one<O>(&mut self) -> Result<O>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <E::DB as sqlx::Database>::Row> + Send + Unpin, 
    {
        return self
            .executor
            .fetch_one(self.sql.clone(), mem::take(&mut self.arguments))
            .await;
    }

    pub async fn fetch_all<O>(&mut self) -> Result<Vec<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <E::DB as sqlx::Database>::Row> + Send + Unpin, 
    {
        return self
            .executor
            .fetch_all(self.sql.clone(), mem::take(&mut self.arguments))
            .await;
    }
}