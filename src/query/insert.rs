use crate::{Entity, Executor, Result, query::Statement, types::{Bindable, QueryResult}};

pub struct Insert<'e, E: Executor> {
    executor: &'e E,
    statement: Statement<E::DB>,
}

impl <'e, E: Executor>Insert<'e, E> {
    pub fn new(table: impl Into<String>, executor: &'e E) -> Self {
        Self {
            executor: executor,
            statement: {
                let mut stmt = Statement::new();
                stmt.table = table.into();
                stmt
            }
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