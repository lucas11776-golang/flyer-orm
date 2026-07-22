use crate::{Entity, Executor, Result, query::Statement, types::{Bindable, QueryResult}};

pub struct Insert<'e, E: Executor> {
    executor: &'e E,
    statement: Statement<E::DB>,
}

impl <'e, E: Executor>Insert<'e, E> {
    pub fn new(table: impl Into<String>, executor: &'e E) -> Self {
        Self {
            executor: executor,
            statement: Statement::new(table)
        }
    }

    pub fn bind<V, O>(&mut self, column: impl Into<String>,  value: V) -> &mut Self
    where
        V: Bindable<E::DB>,
    {
        self
            .statement
            .values
            .insert(column.into(), Box::new(value));
        self
    }

    pub async fn execute<O>(&mut self) -> Result<impl QueryResult>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <E::DB as sqlx::Database>::Row> + Send + Unpin, 
    {
        self
            .executor
            .insert(&self.statement)
            .await
    }

    pub async fn execute_as<O>(&mut self) -> Result<O>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <E::DB as sqlx::Database>::Row> + Send + Unpin, 
    {
        self
            .executor
            .insert_as(&self.statement)
            .await
    }
}