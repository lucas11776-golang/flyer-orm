use std::sync::Arc;

use sqlx::IntoArguments;

use crate::{
    Entity,
    Executor,
    Result,
    query::{Having, Join, Limit, Offset, OrderValue, Pagination, Statement, WhereGroup},
    types::{Bindable, Connector, JoinType, Order, WhereClause},
};

pub struct Query<E: Executor> {
    executor: Arc<E>,
    statement: Statement<E::DB>,
}

impl<E: Executor> Query<E> {
    pub fn new(executor: Arc<E>, table: impl Into<String>) -> Self {
        Self {
            executor,
            statement: Statement::new(table),
        }
    }

    pub fn select<I, S>(mut self, columns: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self
            .statement
            .fields
            .extend(columns.into_iter().map(Into::into));
        self
    }

    fn join_push(
        mut self,
        join_type: JoinType,
        table: impl Into<String>,
        column: impl Into<String>,
        operator: impl Into<String>,
        column_table: impl Into<String>,
    ) -> Self {
        self.statement.join.push(Join {
            table: table.into(),
            column: column.into(),
            operator: operator.into(),
            column_table: column_table.into(),
            join_type,
        });
        self
    }

    pub fn join(
        self,
        table: impl Into<String>,
        column: impl Into<String>,
        operator: impl Into<String>,
        column_table: impl Into<String>,
    ) -> Self {
        self.join_push(JoinType::Join, table, column, operator, column_table)
    }

    pub fn join_right(
        self,
        table: impl Into<String>,
        column: impl Into<String>,
        operator: impl Into<String>,
        column_table: impl Into<String>,
    ) -> Self {
        self.join_push(JoinType::RightJoin, table, column, operator, column_table)
    }

    pub fn join_left(
        self,
        table: impl Into<String>,
        column: impl Into<String>,
        operator: impl Into<String>,
        column_table: impl Into<String>,
    ) -> Self {
        self.join_push(JoinType::LeftJoin, table, column, operator, column_table)
    }

    pub fn join_inner(
        self,
        table: impl Into<String>,
        column: impl Into<String>,
        operator: impl Into<String>,
        column_table: impl Into<String>,
    ) -> Self {
        self.join_push(JoinType::InnerJoin, table, column, operator, column_table)
    }

    pub fn join_full_outer(
        self,
        table: impl Into<String>,
        column: impl Into<String>,
        operator: impl Into<String>,
        column_table: impl Into<String>,
    ) -> Self {
        self.join_push(
            JoinType::FullOuterJoin,
            table,
            column,
            operator,
            column_table,
        )
    }

    pub fn join_cross(
        self,
        table: impl Into<String>,
        column: impl Into<String>,
        operator: impl Into<String>,
        column_table: impl Into<String>,
    ) -> Self {
        self.join_push(JoinType::CrossJoin, table, column, operator, column_table)
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

    pub fn group_by(mut self, column: impl Into<String>) -> Self {
        self.statement.group_by = Some(column.into());
        self
    }

    fn having_push<V>(
        mut self,
        connector: Connector,
        column: impl Into<String>,
        operator: impl Into<String>,
        value: V,
    ) -> Self
    where
        V: Bindable<E::DB>,
    {
        self.statement.having.push(Having {
            column: column.into(),
            operator: operator.into(),
            value: Box::new(value),
            connector,
        });
        self
    }

    pub fn having<V>(
        self,
        column: impl Into<String>,
        operator: impl Into<String>,
        value: V,
    ) -> Self
    where
        V: Bindable<E::DB>,
    {
        self.having_push(Connector::And, column, operator, value)
    }

    #[inline]
    pub fn and_having<V>(
        self,
        column: impl Into<String>,
        operator: impl Into<String>,
        value: V,
    ) -> Self
    where
        V: Bindable<E::DB>,
    {
        self.having_push(Connector::And, column, operator, value)
    }

    pub fn or_having<V>(
        self,
        column: impl Into<String>,
        operator: impl Into<String>,
        value: V,
    ) -> Self
    where
        V: Bindable<E::DB>,
    {
        self.having_push(Connector::Or, column, operator, value)
    }

    pub fn order_by(mut self, column: impl Into<String>, order: Order) -> Self {
        self.statement
            .order_by
            .push(OrderValue::new(column.into(), order));
        self
    }

    pub fn limit(mut self, limit: i64) -> Self
    where
        for<'i> i64: sqlx::Encode<'i, E::DB> + sqlx::Type<E::DB>,
    {
        self.statement.limit = Some(Limit::new(limit));
        self
    }

    pub fn offset(mut self, offset: i64) -> Self
    where
        for<'i> i64: sqlx::Encode<'i, E::DB> + sqlx::Type<E::DB>,
    {
        self.statement.offset = Some(Offset::new(offset));
        self
    }

    pub async fn first<O>(self) -> Result<O>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <E::DB as sqlx::Database>::Row> + Send + Unpin,
        for<'b> <E::DB as sqlx::Database>::Arguments<'b>: IntoArguments<'b, E::DB>,
        for<'c> &'c mut <E::DB as sqlx::Database>::Connection: sqlx::Executor<'c, Database = E::DB>,
    {
        self.executor.first(&self.statement).await
    }

    pub async fn all<O>(self) -> Result<Vec<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <E::DB as sqlx::Database>::Row> + Send + Unpin,
        for<'b> <E::DB as sqlx::Database>::Arguments<'b>: IntoArguments<'b, E::DB>,
        for<'c> &'c mut <E::DB as sqlx::Database>::Connection: sqlx::Executor<'c, Database = E::DB>,
    {
        self.executor.all(&self.statement).await
    }

    pub async fn count(self) -> Result<i64> {
        self.executor.count(&self.statement).await
    }

    pub async fn exists(self) -> Result<bool> {
        self.count().await.map(|c| c > 0)
    }

    pub async fn paginate<O>(mut self, limit: i64, page: i64) -> Result<Pagination<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <E::DB as sqlx::Database>::Row> + Send + Unpin,
        for<'b> <E::DB as sqlx::Database>::Arguments<'b>: IntoArguments<'b, E::DB>,
        for<'c> &'c mut <E::DB as sqlx::Database>::Connection: sqlx::Executor<'c, Database = E::DB>,
        for<'i> i64: sqlx::Encode<'i, E::DB> + sqlx::Type<E::DB>,
    {
        let limit = limit.max(0);
        let page = page.max(1);
        let offset = (page - 1).saturating_mul(limit);

        self.statement.limit = Some(Limit::new(limit));
        self.statement.offset = Some(Offset::new(offset));
        self.statement.page = Some(page);

        self.executor.paginate(&mut self.statement).await
    }
}