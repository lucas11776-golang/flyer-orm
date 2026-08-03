use std::any::Any;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::LazyLock;

pub use database::mysql::MySQL;
pub use database::postgres::Postgres;
pub use database::sqlite::SQLite;
pub use derive::Entity;
pub use executor::Executor;

pub use sqlx::MySqlPool;
pub use sqlx::PgPool;
pub use sqlx::SqlitePool;

use crate::query::update::Update;
use crate::{
    query::{Having, Join, Limit, Offset, OrderValue, Pagination, Statement, WhereGroup, insert::Insert, raw::Raw},
    types::{Bindable, Connector, JoinType, Order, QueryResult, WhereClause},
};

pub mod database;
pub mod query;
pub mod types;
pub mod executor;

pub use anyhow::Result;

pub trait Entity {}

static mut CONTAINER: LazyLock<HashMap<String, Box<dyn Any>>> = LazyLock::new(|| HashMap::new());

pub struct Connection;

impl Connection {
    #[allow(static_mut_refs)]
    pub fn add<E: Executor + 'static>(connection: impl Into<String>, executor: E) {
        unsafe {
            CONTAINER.insert(connection.into(), Box::new(executor));
        }
    }

    #[allow(static_mut_refs)]
    pub fn get<'a, E: Executor + 'static>(connection: impl Into<String>) -> &'a E {
        unsafe {
            CONTAINER
                .get(&connection.into())
                .unwrap()
                .downcast_ref::<E>()
                .unwrap()
        }
    }

    pub fn query<'a, E: Executor + 'static>(connection: impl Into<String>) -> Query<'a, E> {
        Query::new(Self::get(connection))
    }

    pub fn raw<'a, E: Executor + 'static>(connection: impl Into<String>, sql: impl Into<String>) -> Raw<'a, E> {
        Raw::new(Self::get(connection), sql)
    }
}

pub struct Query<'q, E: Executor> {
    executor: &'q E,
    statement: Statement<E::DB>,
}

impl<'q, E: Executor> Query<'q, E> {
    #[inline]
    pub fn new(executor: &'q E, ) -> Self {
        Self {
            executor,
            statement: Statement::new(),
        }
    }

    pub fn table(mut self, table: impl Into<String>) -> Self {
        self.statement.table = table.into();
        self
    }

    #[inline]
    pub fn to_sql(&self) -> String {
        self.executor.to_sql(&self.statement)
    }

    #[inline]
    pub fn raw(&self, sql: impl Into<String>) -> Raw<'_, E> {
        Raw::new(self.executor, sql)
    }

    #[inline]
    pub fn insert(&mut self) -> Insert<'_, E>{
        Insert::new(self.statement.table.clone(), self.executor)
    }

    #[inline]
    pub fn update(&mut self) -> Update<'_, E>{
        Update::new(self.statement.table.clone(), self.executor)
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

    pub async fn first<O>(&self) -> Result<O>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <E::DB as sqlx::Database>::Row> + Send + Unpin
    {
        self
            .executor
            .first(&self.statement)
            .await
    }

    pub async fn all<O>(&self) -> Result<Vec<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <E::DB as sqlx::Database>::Row> + Send + Unpin
    {
        self
            .executor
            .all(&self.statement)
            .await
    }
    

    pub async fn paginate<O>(&mut self, page: i64, limit: i64) -> Result<Pagination<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <E::DB as sqlx::Database>::Row> + Send + Unpin,
        for<'i> i64: sqlx::Encode<'i, E::DB> + sqlx::Type<E::DB>,
    {
        let limit_stable = if limit < 0 { 0 } else { limit };
        let page_stable = if page < 0 { 1 } else { page };
        let offset: i64 = limit_stable * page_stable - limit_stable;

        self.statement.limit = Some(Limit::new(limit_stable));
        self.statement.offset = Some(Offset::new(offset));
        self.statement.page = Some(page); // Should get it using (limit and offset)

        self
            .executor
            .paginate(&mut self.statement)
            .await
    }
}