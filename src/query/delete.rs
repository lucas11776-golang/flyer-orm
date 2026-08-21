use crate::{
    Executor,
    Result,
    query::{Statement, WhereGroup},
    types::{Bindable, Connector, WhereClause}
};

pub struct Delete<'a, E: Executor> {
    executor: &'a E,
    statement: Statement<E::DB>,
}

impl <'a, E: Executor>Delete<'a, E> {
    pub fn new(executor: &'a E, table: impl Into<String>) -> Self {
        Self {
            executor: executor,
            statement: Statement::new(table),
        }
    }

    pub fn r#where<V>(
        &'a mut self,
        column: impl Into<String>,
        operator: impl Into<String>,
        value: V,
    ) -> &'a mut Self
    where
        V: Bindable<E::DB>,
    {
        self.statement.conditions.push(WhereClause::Clause {
            connector: Connector::And,
            column: column.into(),
            operator: operator.into(),
            value: Box::new(value),
        });
        self
    }

    #[inline]
    pub fn and_where<V>(
        &'a mut self,
        column: impl Into<String>,
        operator: impl Into<String>,
        value: V,
    ) -> &'a mut Self
    where
        V: Bindable<E::DB>,
    {
        self.r#where(column, operator, value)
    }

    pub fn or_where<V>(
        &'a mut self,
        column: impl Into<String>,
        operator: impl Into<String>,
        value: V,
    ) -> &'a mut Self
    where
        V: Bindable<E::DB>,
    {
        self.statement.conditions.push(WhereClause::Clause {
            connector: Connector::Or,
            column: column.into(),
            operator: operator.into(),
            value: Box::new(value),
        });
        self
    }

    pub fn where_group<F>(&'a mut self, callback: F) -> &'a mut Self
    where
        F: FnOnce(&mut WhereGroup<E::DB>),
    {
        let mut group = WhereGroup::new();

        callback(&mut group);

        self.statement.conditions.push(WhereClause::Group {
            connector: Connector::And,
            conditions: group.conditions,
        });
        self
    }

    pub async fn execute(self) -> Result<()> {
        self
            .executor
            .delete(&self.statement)
            .await
            .map(|_| {})
    }
}