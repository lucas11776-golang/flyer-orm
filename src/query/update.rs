use std::sync::Arc;

use crate::{Executor, Result, query::Statement, types::Bindable};

pub struct Update<E: Executor> {
    executor: Arc<E>,
    pub(crate) statement: Statement<E::DB>,
}

impl<E: Executor> Update<E> {
    pub fn new(executor: Arc<E>, table: impl Into<String>) -> Self {
        Self {
            executor,
            statement: Statement::new(table),
        }
    }

    pub fn bind<V>(mut self, column: impl Into<String>, value: V) -> Self
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
            .update(&self.statement).await.map(|_| {})
    }
}
