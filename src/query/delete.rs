use std::sync::Arc;

use crate::{Executor, Result, query::Statement};

pub struct Delete<E: Executor> {
    executor: Arc<E>,
    pub(crate) statement: Statement<E::DB>,
}

impl<E: Executor> Delete<E> {
    pub fn new(executor: Arc<E>, table: impl Into<String>) -> Self {
        Self {
            executor: executor,
            statement: Statement::new(table),
        }
    }

    pub async fn execute(self) -> Result<()> {
        self
            .executor
            .delete(&self.statement)
            .await
            .map(|_| {})
    }
}
