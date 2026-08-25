use std::collections::HashMap;

use serde::Serialize;

use crate::{
    executor::Executor,
    query::{delete::Delete, query::Query, update::Update},
    types::{Bindable, Connector, JoinType, Order, WhereClause},
};

pub use crate::Entity;

pub mod delete;
pub mod insert;
pub mod query;
pub mod raw;
pub mod scalar;
pub mod update;

pub struct WhereGroup<DB: sqlx::Database> {
    pub conditions: Vec<WhereClause<DB>>,
}

impl<DB: sqlx::Database> WhereGroup<DB> {
    pub fn new() -> Self {
        return Self {
            conditions: Vec::new(),
        };
    }
}

#[derive(Clone, Debug)]
pub struct Join {
    pub table: String,
    pub column: String,
    pub operator: String,
    pub column_table: String,
    pub join_type: JoinType,
}

pub struct Having<DB: sqlx::Database> {
    pub column: String,
    pub operator: String,
    pub value: Box<dyn Bindable<DB>>,
    pub connector: Connector,
}

#[derive(Clone, Debug)]
pub struct OrderValue {
    pub column: String,
    pub order: Order,
}

impl OrderValue {
    pub fn new(column: impl Into<String>, order: Order) -> Self {
        Self {
            column: column.into(),
            order: order,
        }
    }
}

pub struct Limit<DB: sqlx::Database> {
    pub value: Box<dyn Bindable<DB>>,
}

impl<DB: sqlx::Database> Limit<DB> {
    pub fn new(limit: i64) -> Self
    where
        for<'i> i64: sqlx::Encode<'i, DB> + sqlx::Type<DB>,
    {
        Self {
            value: Box::new(limit),
        }
    }
}

pub struct Offset<DB: sqlx::Database> {
    pub value: Box<dyn Bindable<DB>>,
}

impl<DB: sqlx::Database> Offset<DB> {
    pub fn new(offset: i64) -> Self
    where
        for<'i> i64: sqlx::Encode<'i, DB> + sqlx::Type<DB>,
    {
        Self {
            value: Box::new(offset),
        }
    }
}

#[derive(Default)]
pub struct Statement<DB: sqlx::Database> {
    pub table: String,
    pub fields: Vec<String>,
    pub join: Vec<Join>,
    pub conditions: Vec<WhereClause<DB>>,
    pub group_by: Option<String>,
    pub having: Vec<Having<DB>>,
    pub order_by: Vec<OrderValue>,
    pub limit: Option<Limit<DB>>,
    pub offset: Option<Offset<DB>>,
    pub page: Option<i64>,
    pub values: HashMap<String, Box<dyn Bindable<DB>>>,
}

impl<DB: sqlx::Database> Statement<DB> {
    pub fn new(table: impl Into<String>) -> Self {
        Self {
            table: table.into(),
            fields: Vec::new(),
            join: Vec::new(),
            conditions: Vec::new(),
            group_by: None,
            having: Vec::new(),
            order_by: Vec::new(),
            limit: None,
            offset: None,
            page: None,
            values: HashMap::new(),
        }
    }
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct Pagination<Entity> {
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub items: Vec<Entity>,
}

macro_rules! impl_where_clause_methods {
    ($target:ident) => {
        impl<E: Executor> $target<E> {
            fn push_where(mut self, clause: WhereClause<E::DB>) -> Self {
                self
                    .statement
                    .conditions
                    .push(clause);
                self
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
                self.push_where(WhereClause::Clause {
                    connector: Connector::And,
                    column: column.into(),
                    operator: operator.into(),
                    value: Box::new(value),
                })
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
                self.push_where(WhereClause::Clause {
                    connector: Connector::Or,
                    column: column.into(),
                    operator: operator.into(),
                    value: Box::new(value),
                })
            }

            pub fn where_null(self, column: impl Into<String>, is_null: bool) -> Self {
                self.push_where(WhereClause::NullCheck {
                    column: column.into(),
                    is_null,
                    connector: Connector::And,
                })
            }

            #[inline]
            pub fn and_where_null(self, column: impl Into<String>, is_null: bool) -> Self {
                self.where_null(column, is_null)
            }

            pub fn or_where_null(self, column: impl Into<String>, is_null: bool) -> Self {
                self.push_where(WhereClause::NullCheck {
                    column: column.into(),
                    is_null,
                    connector: Connector::Or,
                })
            }

            pub fn where_not_null(self, column: impl Into<String>) -> Self {
                self.where_null(column, false)
            }

            #[inline]
            pub fn and_where_not_null(self, column: impl Into<String>) -> Self {
                self.where_not_null(column)
            }

            pub fn or_where_not_null(self, column: impl Into<String>) -> Self {
                self.or_where_null(column, false)
            }

            pub fn where_in<V>(self, column: impl Into<String>, items: Vec<V>) -> Self
            where
                V: Bindable<E::DB> + 'static,
            {
                self.push_where(WhereClause::In {
                    column: column.into(),
                    negated: false,
                    values: items
                        .into_iter()
                        .map(|v| Box::new(v) as Box<dyn Bindable<E::DB>>)
                        .collect(),
                    connector: Connector::And,
                })
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
                self.push_where(WhereClause::In {
                    column: column.into(),
                    negated: false,
                    values: items
                        .into_iter()
                        .map(|v| Box::new(v) as Box<dyn Bindable<E::DB>>)
                        .collect(),
                    connector: Connector::Or,
                })
            }

            pub fn where_not_in<V>(self, column: impl Into<String>, items: Vec<V>) -> Self
            where
                V: Bindable<E::DB> + 'static,
            {
                self.push_where(WhereClause::In {
                    column: column.into(),
                    negated: true,
                    values: items
                        .into_iter()
                        .map(|v| Box::new(v) as Box<dyn Bindable<E::DB>>)
                        .collect(),
                    connector: Connector::And,
                })
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
                self.push_where(WhereClause::In {
                    column: column.into(),
                    negated: true,
                    values: items
                        .into_iter()
                        .map(|v| Box::new(v) as Box<dyn Bindable<E::DB>>)
                        .collect(),
                    connector: Connector::Or,
                })
            }

            pub fn where_between<V>(self, column: impl Into<String>, start: V, end: V) -> Self
            where
                V: Bindable<E::DB>,
            {
                self.push_where(WhereClause::Between {
                    column: column.into(),
                    negated: false,
                    low: Box::new(start),
                    high: Box::new(end),
                    connector: Connector::And,
                })
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
            pub fn and_where_in_between<V>(
                self,
                column: impl Into<String>,
                start: V,
                end: V,
            ) -> Self
            where
                V: Bindable<E::DB>,
            {
                self.where_between(column, start, end)
            }

            pub fn or_where_between<V>(self, column: impl Into<String>, start: V, end: V) -> Self
            where
                V: Bindable<E::DB>,
            {
                self.push_where(WhereClause::Between {
                    column: column.into(),
                    negated: false,
                    low: Box::new(start),
                    high: Box::new(end),
                    connector: Connector::Or,
                })
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
                self.push_where(WhereClause::Between {
                    column: column.into(),
                    negated: true,
                    low: Box::new(start),
                    high: Box::new(end),
                    connector: Connector::And,
                })
            }

            #[inline]
            pub fn and_where_not_between<V>(
                self,
                column: impl Into<String>,
                start: V,
                end: V,
            ) -> Self
            where
                V: Bindable<E::DB>,
            {
                self.where_not_between(column, start, end)
            }

            pub fn or_where_not_between<V>(
                self,
                column: impl Into<String>,
                start: V,
                end: V,
            ) -> Self
            where
                V: Bindable<E::DB>,
            {
                self.push_where(WhereClause::Between {
                    column: column.into(),
                    negated: true,
                    low: Box::new(start),
                    high: Box::new(end),
                    connector: Connector::Or,
                })
            }

            pub fn where_group<F>(self, callback: F) -> Self
            where
                F: FnOnce(&mut WhereGroup<E::DB>),
            {
                let mut group = WhereGroup::new();

                callback(&mut group);

                self.push_where(WhereClause::Group {
                    connector: Connector::And,
                    conditions: group.conditions,
                })
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
                let mut group = WhereGroup::new();

                callback(&mut group);

                self.push_where(WhereClause::Group {
                    connector: Connector::Or,
                    conditions: group.conditions,
                })
            }
        }
    };
}

impl_where_clause_methods!(Delete);
impl_where_clause_methods!(Update);
impl_where_clause_methods!(Query);

macro_rules! impl_mut_where_clause_methods {
    ($target:ident) => {
        impl<DB: sqlx::Database> $target<DB> {
            fn push_where(&mut self, clause: WhereClause<DB>) -> &mut Self {
                self
                    .conditions
                    .push(clause);
                self
            }

            pub fn r#where<V>(&mut self, column: impl Into<String>, operator: impl Into<String>, value: V) -> &mut Self
            where
                V: Bindable<DB>,
            {
                self.push_where(WhereClause::Clause {
                    connector: Connector::And,
                    column: column.into(),
                    operator: operator.into(),
                    value: Box::new(value),
                })
            }

            #[inline]
            pub fn and_where<V>(&mut self, column: impl Into<String>, operator: impl Into<String>, value: V) -> &mut Self
            where
                V: Bindable<DB>,
            {
                self.r#where(column, operator, value)
            }

            pub fn or_where<V>(&mut self, column: impl Into<String>, operator: impl Into<String>, value: V) -> &mut Self
            where
                V: Bindable<DB>,
            {
                self.push_where(WhereClause::Clause {
                    connector: Connector::Or,
                    column: column.into(),
                    operator: operator.into(),
                    value: Box::new(value),
                })
            }

            pub fn where_null(&mut self, column: impl Into<String>, is_null: bool) -> &mut Self {
                self.push_where(WhereClause::NullCheck {
                    column: column.into(),
                    is_null,
                    connector: Connector::And,
                })
            }

            #[inline]
            pub fn and_where_null(&mut self, column: impl Into<String>, is_null: bool) -> &mut Self {
                self.where_null(column, is_null)
            }

            pub fn or_where_null(&mut self, column: impl Into<String>, is_null: bool) -> &mut Self {
                self.push_where(WhereClause::NullCheck {
                    column: column.into(),
                    is_null,
                    connector: Connector::Or,
                })
            }

            pub fn where_not_null(&mut self, column: impl Into<String>) -> &mut Self {
                self.where_null(column, false)
            }

            #[inline]
            pub fn and_where_not_null(&mut self, column: impl Into<String>) -> &mut Self {
                self.where_not_null(column)
            }

            pub fn or_where_not_null(&mut self, column: impl Into<String>) -> &mut Self {
                self.or_where_null(column, false)
            }

            pub fn where_in<V>(&mut self, column: impl Into<String>, items: Vec<V>) -> &mut Self
            where
                V: Bindable<DB> + 'static,
            {
                self.push_where(WhereClause::In {
                    column: column.into(),
                    negated: false,
                    values: items
                        .into_iter()
                        .map(|v| Box::new(v) as Box<dyn Bindable<DB>>)
                        .collect(),
                    connector: Connector::And,
                })
            }

            #[inline]
            pub fn and_where_in<V>(&mut self, column: impl Into<String>, items: Vec<V>) -> &mut Self
            where
                V: Bindable<DB> + 'static,
            {
                self.where_in(column, items)
            }

            pub fn or_where_in<V>(&mut self, column: impl Into<String>, items: Vec<V>) -> &mut Self
            where
                V: Bindable<DB> + 'static,
            {
                self.push_where(WhereClause::In {
                    column: column.into(),
                    negated: false,
                    values: items
                        .into_iter()
                        .map(|v| Box::new(v) as Box<dyn Bindable<DB>>)
                        .collect(),
                    connector: Connector::Or,
                })
            }

            pub fn where_not_in<V>(&mut self, column: impl Into<String>, items: Vec<V>) -> &mut Self
            where
                V: Bindable<DB> + 'static,
            {
                self.push_where(WhereClause::In {
                    column: column.into(),
                    negated: true,
                    values: items
                        .into_iter()
                        .map(|v| Box::new(v) as Box<dyn Bindable<DB>>)
                        .collect(),
                    connector: Connector::And,
                })
            }

            #[inline]
            pub fn and_where_not_in<V>(&mut self, column: impl Into<String>, items: Vec<V>) -> &mut Self
            where
                V: Bindable<DB> + 'static,
            {
                self.where_not_in(column, items)
            }

            pub fn or_where_not_in<V>(&mut self, column: impl Into<String>, items: Vec<V>) -> &mut Self
            where
                V: Bindable<DB> + 'static,
            {
                self.push_where(WhereClause::In {
                    column: column.into(),
                    negated: true,
                    values: items
                        .into_iter()
                        .map(|v| Box::new(v) as Box<dyn Bindable<DB>>)
                        .collect(),
                    connector: Connector::Or,
                })
            }

            pub fn where_between<V>(&mut self, column: impl Into<String>, start: V, end: V) -> &mut Self
            where
                V: Bindable<DB>,
            {
                self.push_where(WhereClause::Between {
                    column: column.into(),
                    negated: false,
                    low: Box::new(start),
                    high: Box::new(end),
                    connector: Connector::And,
                })
            }

            #[inline]
            pub fn where_in_between<V>(&mut self, column: impl Into<String>, start: V, end: V) -> &mut Self
            where
                V: Bindable<DB>,
            {
                self.where_between(column, start, end)
            }

            #[inline]
            pub fn and_where_between<V>(&mut self, column: impl Into<String>, start: V, end: V) -> &mut Self
            where
                V: Bindable<DB>,
            {
                self.where_between(column, start, end)
            }

            #[inline]
            pub fn and_where_in_between<V>(&mut self, column: impl Into<String>, start: V, end: V) -> &mut Self
            where
                V: Bindable<DB>,
            {
                self.where_between(column, start, end)
            }

            pub fn or_where_between<V>(&mut self, column: impl Into<String>, start: V, end: V) -> &mut Self
            where
                V: Bindable<DB>,
            {
                self.push_where(WhereClause::Between {
                    column: column.into(),
                    negated: false,
                    low: Box::new(start),
                    high: Box::new(end),
                    connector: Connector::Or,
                })
            }

            #[inline]
            pub fn or_where_in_between<V>(&mut self, column: impl Into<String>, start: V, end: V) -> &mut Self
            where
                V: Bindable<DB>,
            {
                self.or_where_between(column, start, end)
            }

            pub fn where_not_between<V>(&mut self, column: impl Into<String>, start: V, end: V) -> &mut Self
            where
                V: Bindable<DB>,
            {
                self.push_where(WhereClause::Between {
                    column: column.into(),
                    negated: true,
                    low: Box::new(start),
                    high: Box::new(end),
                    connector: Connector::And,
                })
            }

            #[inline]
            pub fn and_where_not_between<V>(&mut self, column: impl Into<String>, start: V, end: V) -> &mut Self
            where
                V: Bindable<DB>,
            {
                self.where_not_between(column, start, end)
            }

            pub fn or_where_not_between<V>(&mut self, column: impl Into<String>, start: V, end: V) -> &mut Self
            where
                V: Bindable<DB>,
            {
                self.push_where(WhereClause::Between {
                    column: column.into(),
                    negated: true,
                    low: Box::new(start),
                    high: Box::new(end),
                    connector: Connector::Or,
                })
            }

            pub fn where_group<F>(&mut self, callback: F) -> &mut Self
            where
                F: FnOnce(&mut WhereGroup<DB>),
            {
                let mut group = WhereGroup::new();
                callback(&mut group);

                self.push_where(WhereClause::Group {
                    connector: Connector::And,
                    conditions: group.conditions,
                })
            }

            #[inline]
            pub fn and_where_group<F>(&mut self, callback: F) -> &mut Self
            where
                F: FnOnce(&mut WhereGroup<DB>),
            {
                self.where_group(callback)
            }

            pub fn or_where_group<F>(&mut self, callback: F) -> &mut Self
            where
                F: FnOnce(&mut WhereGroup<DB>),
            {
                let mut group = WhereGroup::new();
                callback(&mut group);

                self.push_where(WhereClause::Group {
                    connector: Connector::Or,
                    conditions: group.conditions,
                })
            }
        }
    }
}

impl_mut_where_clause_methods!(WhereGroup);