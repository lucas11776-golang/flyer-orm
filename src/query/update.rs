use crate::{
    Executor,
    Result,
    query::{Statement, WhereGroup},
    types::{Bindable, Connector, WhereClause}
};

pub struct Update<'e, E: Executor> {
    executor: &'e E,
    statement: Statement<E::DB>,
}

impl <'e, E: Executor>Update<'e, E> {
    pub fn new(executor: &'e E, table: impl Into<String>) -> Self {
        Self {
            executor: executor,
            statement: Statement::new(table),
        }
    }

    pub fn r#where<V>(
        &mut self,
        column: impl Into<String>,
        operator: impl Into<String>,
        value: V,
    ) -> &mut Self
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
        &mut self,
        column: impl Into<String>,
        operator: impl Into<String>,
        value: V,
    ) -> &mut Self
    where
        V: Bindable<E::DB>,
    {
        self.r#where(column, operator, value)
    }

    pub fn or_where<V>(
        &mut  self,
        column: impl Into<String>,
        operator: impl Into<String>,
        value: V,
    ) -> &mut Self
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

    pub fn where_group<F>(&mut self, callback: F) -> &mut Self
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

    pub fn bind<V>(&mut self, column: impl Into<String>,  value: V) -> &mut Self
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
            .update(&self.statement)
            .await
            .map(|_| {})
    }
}