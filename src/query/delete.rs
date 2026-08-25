use std::sync::Arc;

use crate::{
    Executor,
    Result,
    query::{Statement, WhereGroup},
    types::{Bindable, Connector, WhereClause},
};

pub struct Delete<E: Executor> {
    executor: Arc<E>,
    statement: Statement<E::DB>,
}

impl<E: Executor> Delete<E> {
    pub fn new(executor: Arc<E>, table: impl Into<String>) -> Self {
        Self {
            executor: executor,
            statement: Statement::new(table),
        }
    }

    fn where_push(mut self, clause: WhereClause<E::DB>) -> Self {
        self
            .statement
            .conditions
            .push(clause);
        self
    }

    fn where_clause_push<V>(
        self,
        connector: Connector,
        column: impl Into<String>,
        operator: impl Into<String>,
        value: V,
    ) -> Self
    where
        V: Bindable<E::DB>,
    {
        self.where_push(WhereClause::Clause {
            connector,
            column: column.into(),
            operator: operator.into(),
            value: Box::new(value),
        })
    }

    fn where_null_push(
        self,
        connector: Connector,
        column: impl Into<String>,
        is_null: bool,
    ) -> Self {
        self.where_push(WhereClause::NullCheck {
            column: column.into(),
            is_null,
            connector,
        })
    }

    fn where_in_push<V>(
        self,
        connector: Connector,
        column: impl Into<String>,
        items: Vec<V>,
        negated: bool,
    ) -> Self
    where
        V: Bindable<E::DB> + 'static,
    {
        self.where_push(WhereClause::In {
            column: column.into(),
            negated,
            values: items
                .into_iter()
                .map(|v| Box::new(v) as Box<dyn Bindable<E::DB>>)
                .collect(),
            connector,
        })
    }

    fn where_between_push<V>(
        self,
        connector: Connector,
        column: impl Into<String>,
        start: V,
        end: V,
        negated: bool,
    ) -> Self
    where
        V: Bindable<E::DB>,
    {
        self.where_push(WhereClause::Between {
            column: column.into(),
            negated,
            low: Box::new(start),
            high: Box::new(end),
            connector,
        })
    }

    fn where_group_push<F>(self, connector: Connector, callback: F) -> Self
    where
        F: FnOnce(&mut WhereGroup<E::DB>),
    {
        let mut group = WhereGroup::new();
        
        callback(&mut group);

        self.where_push(WhereClause::Group {
            connector,
            conditions: group.conditions,
        })
    }

    pub fn r#where<V>(
        self,
        column: impl Into<String>,
        operator: impl Into<String>,
        value: V,
    ) -> Self
    where
        V: Bindable<E::DB>,
    {
        self.where_clause_push(Connector::And, column, operator, value)
    }

    #[inline]
    pub fn and_where<V>(
        self,
        column: impl Into<String>,
        operator: impl Into<String>,
        value: V,
    ) -> Self
    where
        V: Bindable<E::DB>,
    {
        self.r#where(column, operator, value)
    }

    pub fn or_where<V>(
        self,
        column: impl Into<String>,
        operator: impl Into<String>,
        value: V,
    ) -> Self
    where
        V: Bindable<E::DB>,
    {
        self.where_clause_push(Connector::Or, column, operator, value)
    }

    pub fn where_null(self, column: impl Into<String>, is_null: bool) -> Self {
        self.where_null_push(Connector::And, column, is_null)
    }

    #[inline]
    pub fn and_where_null(self, column: impl Into<String>, is_null: bool) -> Self {
        self.where_null(column, is_null)
    }

    pub fn or_where_null(self, column: impl Into<String>, is_null: bool) -> Self {
        self.where_null_push(Connector::Or, column, is_null)
    }

    pub fn where_not_null(self, column: impl Into<String>) -> Self {
        self.where_null_push(Connector::And, column, false)
    }

    #[inline]
    pub fn and_where_not_null(self, column: impl Into<String>) -> Self {
        self.where_not_null(column)
    }

    pub fn or_where_not_null(self, column: impl Into<String>) -> Self {
        self.where_null_push(Connector::Or, column, false)
    }

    pub fn where_in<V>(self, column: impl Into<String>, items: Vec<V>) -> Self
    where
        V: Bindable<E::DB> + 'static,
    {
        self.where_in_push(Connector::And, column, items, false)
    }

    #[inline]
    pub fn and_where_in<V>(self, column: impl Into<String>, items: Vec<V>) -> Self
    where
        V: Bindable<E::DB> + 'static,
    {
        self.where_in(column, items)
    }

    pub fn or_where_in<V>(self, column: impl Into<String>, items: Vec<V>) -> Self
    where
        V: Bindable<E::DB> + 'static,
    {
        self.where_in_push(Connector::Or, column, items, false)
    }

    pub fn where_not_in<V>(self, column: impl Into<String>, items: Vec<V>) -> Self
    where
        V: Bindable<E::DB> + 'static,
    {
        self.where_in_push(Connector::And, column, items, true)
    }

    #[inline]
    pub fn and_where_not_in<V>(self, column: impl Into<String>, items: Vec<V>) -> Self
    where
        V: Bindable<E::DB> + 'static,
    {
        self.where_not_in(column, items)
    }

    pub fn or_where_not_in<V>(self, column: impl Into<String>, items: Vec<V>) -> Self
    where
        V: Bindable<E::DB> + 'static,
    {
        self.where_in_push(Connector::Or, column, items, true)
    }

    pub fn where_between<V>(self, column: impl Into<String>, start: V, end: V) -> Self
    where
        V: Bindable<E::DB>,
    {
        self.where_between_push(Connector::And, column, start, end, false)
    }

    #[inline]
    pub fn where_in_between<V>(self, column: impl Into<String>, start: V, end: V) -> Self
    where
        V: Bindable<E::DB>,
    {
        self.where_between(column, start, end)
    }

    #[inline]
    pub fn and_where_between<V>(self, column: impl Into<String>, start: V, end: V) -> Self
    where
        V: Bindable<E::DB>,
    {
        self.where_between(column, start, end)
    }

    #[inline]
    pub fn and_where_in_between<V>(self, column: impl Into<String>, start: V, end: V) -> Self
    where
        V: Bindable<E::DB>,
    {
        self.where_between(column, start, end)
    }

    pub fn or_where_between<V>(self, column: impl Into<String>, start: V, end: V) -> Self
    where
        V: Bindable<E::DB>,
    {
        self.where_between_push(Connector::Or, column, start, end, false)
    }

    #[inline]
    pub fn or_where_in_between<V>(self, column: impl Into<String>, start: V, end: V) -> Self
    where
        V: Bindable<E::DB>,
    {
        self.or_where_between(column, start, end)
    }

    pub fn where_not_between<V>(self, column: impl Into<String>, start: V, end: V) -> Self
    where
        V: Bindable<E::DB>,
    {
        self.where_between_push(Connector::And, column, start, end, true)
    }

    #[inline]
    pub fn and_where_not_between<V>(self, column: impl Into<String>, start: V, end: V) -> Self
    where
        V: Bindable<E::DB>,
    {
        self.where_not_between(column, start, end)
    }

    pub fn or_where_not_between<V>(self, column: impl Into<String>, start: V, end: V) -> Self
    where
        V: Bindable<E::DB>,
    {
        self.where_between_push(Connector::Or, column, start, end, true)
    }

    pub fn where_group<F>(self, callback: F) -> Self
    where
        F: FnOnce(&mut WhereGroup<E::DB>),
    {
        self.where_group_push(Connector::And, callback)
    }

    #[inline]
    pub fn and_where_group<F>(self, callback: F) -> Self
    where
        F: FnOnce(&mut WhereGroup<E::DB>),
    {
        self.where_group(callback)
    }

    pub fn or_where_group<F>(self, callback: F) -> Self
    where
        F: FnOnce(&mut WhereGroup<E::DB>),
    {
        self.where_group_push(Connector::Or, callback)
    }

    pub async fn execute(self) -> Result<()> {
        self.executor.delete(&self.statement).await.map(|_| {})
    }
}