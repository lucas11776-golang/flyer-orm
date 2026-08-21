use std::sync::Arc;

use crate::{Entity, Executor, Result, query::Statement, types::Bindable};

pub struct Insert<E: Executor> {
    executor: Arc<E>,
    statement: Statement<E::DB>,
}

impl <E: Executor>Insert<E> {
    pub fn new(executor: Arc<E>, table: impl Into<String>) -> Self {
        Self {
            executor: executor,
            statement: Statement::new(table),
        }
    }

    pub fn bind<V>(mut self, column: impl Into<String>,  value: V) -> Self
    where
        V: Bindable<E::DB>,
    {
        self
            .statement
            .values
            .insert(column.into(), Box::new(value));
        self
    }

    pub async fn execute(self) -> Result<()> {
        self
            .executor
            .insert(&self.statement)
            .await
            .map(|_| {})
    }

    pub async fn execute_as<O>(self) -> Result<O>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <E::DB as sqlx::Database>::Row> + Send + Unpin, 
    {
        self
            .executor
            .insert_as(&self.statement)
            .await
    }
}