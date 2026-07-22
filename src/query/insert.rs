use std::mem;

use crate::{Entity, Executor, Result, query::Statement, types::{Bindable, QueryResult}};

pub struct RawQuery<'e, E: Executor> {
    executor: &'e E,
    statement: &'e mut Statement<E::DB>,
}

impl <'e, E: Executor>RawQuery<'e, E> {
    pub fn new(executor: &'e E, statement: &'e mut Statement<E::DB>) -> Self {
        Self {
            executor: executor,
            statement: statement
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
            .insert(self.statement)
            .await
    }


    // pub async fn execute<O>(&mut self) -> Result<impl QueryResult>
    // where
    //     O: Entity + for<'r> sqlx::FromRow<'r, <E::DB as sqlx::Database>::Row> + Send + Unpin, 
    // {
    //     self
    //         .executor
    //         .execute(self.sql.clone(), mem::take(&mut self.arguments))
    //         .await
    // }

    // pub async fn first<O>(&mut self) -> Result<O>
    // where
    //     O: Entity + for<'r> sqlx::FromRow<'r, <E::DB as sqlx::Database>::Row> + Send + Unpin, 
    // {
    //     self
    //         .executor
    //         .fetch_one(self.sql.clone(), mem::take(&mut self.arguments))
    //         .await
    // }

    // pub async fn all<O>(&mut self) -> Result<Vec<O>>
    // where
    //     O: Entity + for<'r> sqlx::FromRow<'r, <E::DB as sqlx::Database>::Row> + Send + Unpin, 
    // {
    //     self
    //         .executor
    //         .fetch_all(self.sql.clone(), mem::take(&mut self.arguments))
    //         .await
    // }
}