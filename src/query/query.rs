use std::sync::Arc;

use sqlx::IntoArguments;

use crate::{
    Entity,
    Executor,
    Result,
    query::{Having, Join, Limit, Offset, OrderValue, Pagination, Statement, WhereGroup},
    types::{Bindable, Connector, JoinType, Order, WhereClause}
};
pub struct Query<E: Executor> {
    executor: Arc<E>,
    statement: Statement<E::DB>,
}

impl <E: Executor>Query<E> {
    pub fn new(executor: Arc<E>, table: impl Into<String>) -> Self {
        Self {
            executor: executor,
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
            .extend(columns.into_iter()
            .map(Into::into));
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

    pub fn r#where<V>(
        mut self,
        column: impl Into<String>,
        operator: impl Into<String>,
        value: V,
    ) -> Self
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
        mut self,
        column: impl Into<String>,
        operator: impl Into<String>,
        value: V,
    ) -> Self
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

    pub fn where_group<F>(mut self, callback: F) -> Self
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
            connector: connector,
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
        for <'b> <E::DB as sqlx::Database>::Arguments<'b>: IntoArguments<'b, E::DB>,
        for<'c> &'c mut <E::DB as sqlx::Database>::Connection: sqlx::Executor<'c, Database = E::DB>,
    {
        self
            .executor
            .first(&self.statement)
            .await
    }

    pub async fn all<O>(self) -> Result<Vec<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <E::DB as sqlx::Database>::Row> + Send + Unpin,
        for <'b> <E::DB as sqlx::Database>::Arguments<'b>: IntoArguments<'b, E::DB>,
        for<'c> &'c mut <E::DB as sqlx::Database>::Connection: sqlx::Executor<'c, Database = E::DB>,
    {
        self
            .executor
            .all(&self.statement)
            .await
    }

    pub async fn count(self) -> Result<i64> {
        self
            .executor
            .count(&self.statement)
            .await
    }

    pub async fn exists(self) -> Result<bool> {
        self
            .count()
            .await
            .map(|c| c > 0)
    }

    pub async fn paginate<O>(mut self, limit: i64, page: i64) -> Result<Pagination<O>>
    where 
        O: Entity + for<'r> sqlx::FromRow<'r, <E::DB as sqlx::Database>::Row> + Send + Unpin,
        for <'b> <E::DB as sqlx::Database>::Arguments<'b>: IntoArguments<'b, E::DB>,
        for<'c> &'c mut <E::DB as sqlx::Database>::Connection: sqlx::Executor<'c, Database = E::DB>,
        for<'i> i64: sqlx::Encode<'i, E::DB> + sqlx::Type<E::DB>,
    {
        let limit = limit.max(0);
        let page = page.max(1);
        let offset = (page - 1).saturating_mul(limit);

        self.statement.limit = Some(Limit::new(limit));
        self.statement.offset = Some(Offset::new(offset));
        self.statement.page = Some(page);

        self
            .executor
            .paginate(&mut self.statement)
            .await
    }
}