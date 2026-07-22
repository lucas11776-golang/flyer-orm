use std::borrow::Cow;
use std::error::Error;

pub use database::mysql::MySQL;
pub use database::postgres::Postgres;
pub use database::sqlite::SQLite;
pub use derive::Entity;
pub use executor::Executor;

use crate::{
    query::{raw::Raw, Having, Join, Limit, Offset, OrderValue, Pagination, Statement, WhereGroup},
    types::{Bindable, Connector, JoinType, Order, QueryResult, WhereClause},
};

pub mod database;
pub mod query;
pub mod types;
pub mod executor;

pub type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

pub trait Entity {}

pub struct Query<'q, E: Executor> {
    executor: &'q E,
    statement: Statement<E::DB>,
}

impl<'q, E: Executor> Query<'q, E> {
    #[inline]
    pub fn new(executor: &'q E, table: impl Into<Cow<'q, str>>) -> Self {
        Self {
            executor,
            statement: Statement::new(table.into().into_owned()),
        }
    }

    #[inline]
    pub fn to_sql(&self) -> String {
        self.executor.to_sql(&self.statement)
    }

    #[inline]
    pub fn raw(&self, sql: impl Into<String>) -> Raw<'_, E> {
        Raw::new(self.executor, sql)
    }

    fn join_push(
        mut self,
        join_type: JoinType,
        table: impl Into<Cow<'q, str>>,
        column: impl Into<Cow<'q, str>>,
        operator: impl Into<Cow<'q, str>>,
        column_table: impl Into<Cow<'q, str>>,
    ) -> Self {
        self.statement.join.push(Join {
            table: table.into().into_owned(),
            column: column.into().into_owned(),
            operator: operator.into().into_owned(),
            column_table: column_table.into().into_owned(),
            join_type,
        });
        self
    }

    pub fn join(
        self,
        table: impl Into<Cow<'q, str>>,
        column: impl Into<Cow<'q, str>>,
        operator: impl Into<Cow<'q, str>>,
        column_table: impl Into<Cow<'q, str>>,
    ) -> Self {
        self.join_push(JoinType::Join, table, column, operator, column_table)
    }

    pub fn join_right(
        self,
        table: impl Into<Cow<'q, str>>,
        column: impl Into<Cow<'q, str>>,
        operator: impl Into<Cow<'q, str>>,
        column_table: impl Into<Cow<'q, str>>,
    ) -> Self {
        self.join_push(JoinType::RightJoin, table, column, operator, column_table)
    }

    pub fn join_left(
        self,
        table: impl Into<Cow<'q, str>>,
        column: impl Into<Cow<'q, str>>,
        operator: impl Into<Cow<'q, str>>,
        column_table: impl Into<Cow<'q, str>>,
    ) -> Self {
        self.join_push(JoinType::LeftJoin, table, column, operator, column_table)
    }

    pub fn join_inner(
        self,
        table: impl Into<Cow<'q, str>>,
        column: impl Into<Cow<'q, str>>,
        operator: impl Into<Cow<'q, str>>,
        column_table: impl Into<Cow<'q, str>>,
    ) -> Self {
        self.join_push(JoinType::InnerJoin, table, column, operator, column_table)
    }

    pub fn join_full_outer(
        self,
        table: impl Into<Cow<'q, str>>,
        column: impl Into<Cow<'q, str>>,
        operator: impl Into<Cow<'q, str>>,
        column_table: impl Into<Cow<'q, str>>,
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
        table: impl Into<Cow<'q, str>>,
        column: impl Into<Cow<'q, str>>,
        operator: impl Into<Cow<'q, str>>,
        column_table: impl Into<Cow<'q, str>>,
    ) -> Self {
        self.join_push(JoinType::CrossJoin, table, column, operator, column_table)
    }

    pub fn r#where<V>(
        mut self,
        column: impl Into<Cow<'q, str>>,
        operator: impl Into<Cow<'q, str>>,
        value: V,
    ) -> Self
    where
        V: Bindable<E::DB>,
    {
        self.statement.conditions.push(WhereClause::Clause {
            connector: Connector::And,
            column: column.into().into_owned(),
            operator: operator.into().into_owned(),
            value: Box::new(value),
        });
        self
    }

    #[inline]
    pub fn and_where<V>(
        self,
        column: impl Into<Cow<'q, str>>,
        operator: impl Into<Cow<'q, str>>,
        value: V,
    ) -> Self
    where
        V: Bindable<E::DB>,
    {
        self.r#where(column, operator, value)
    }

    pub fn or_where<V>(
        mut self,
        column: impl Into<Cow<'q, str>>,
        operator: impl Into<Cow<'q, str>>,
        value: V,
    ) -> Self
    where
        V: Bindable<E::DB>,
    {
        self.statement.conditions.push(WhereClause::Clause {
            connector: Connector::Or,
            column: column.into().into_owned(),
            operator: operator.into().into_owned(),
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

    pub fn group_by(mut self, column: impl Into<Cow<'q, str>>) -> Self {
        self.statement.group_by = Some(column.into().into_owned());
        self
    }

    fn having_push<V>(
        mut self,
        connector: Connector,
        column: impl Into<Cow<'q, str>>,
        operator: impl Into<Cow<'q, str>>,
        value: V,
    ) -> Self
    where
        V: Bindable<E::DB>,
    {
        self.statement.having.push(Having {
            column: column.into().into_owned(),
            operator: operator.into().into_owned(),
            value: Box::new(value),
            connector,
        });
        self
    }

    pub fn having<V>(
        self,
        column: impl Into<Cow<'q, str>>,
        operator: impl Into<Cow<'q, str>>,
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
        column: impl Into<Cow<'q, str>>,
        operator: impl Into<Cow<'q, str>>,
        value: V,
    ) -> Self
    where
        V: Bindable<E::DB>,
    {
        self.having_push(Connector::And, column, operator, value)
    }

    pub fn or_having<V>(
        self,
        column: impl Into<Cow<'q, str>>,
        operator: impl Into<Cow<'q, str>>,
        value: V,
    ) -> Self
    where
        V: Bindable<E::DB>,
    {
        self.having_push(Connector::Or, column, operator, value)
    }

    pub fn order_by(mut self, column: impl Into<Cow<'q, str>>, order: Order) -> Self {
        self.statement
            .order_by
            .push(OrderValue::new(column.into().into_owned(), order));
        self
    }

    pub fn limit<V>(mut self, limit: V) -> Self
    where
        V: Bindable<E::DB>,
    {
        self.statement.limit = Some(Limit {
            value: Box::new(limit),
        });
        self
    }

    pub fn offset<V>(mut self, offset: V) -> Self
    where
        V: Bindable<E::DB>,
    {
        self.statement.offset = Some(Offset {
            value: Box::new(offset),
        });
        self
    }

    pub async fn first<O>(&self) -> Result<O>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <E::DB as sqlx::Database>::Row> + Send + Unpin,
    {
        self
            .executor
            .first(&self.statement)
            .await
    }

    pub async fn get<O>(&self) -> Result<Vec<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <E::DB as sqlx::Database>::Row> + Send + Unpin,
    {
        self
            .executor
            .all(&self.statement)
            .await
    }
}