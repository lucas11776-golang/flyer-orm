use std::collections::HashMap;

use serde::Serialize;

use crate::{
    WhereClause,
    types::{Bindable, Connector, JoinType, Order}
};

pub use crate::Entity;

pub mod insert;
pub mod raw;

pub struct WhereGroup<DB: sqlx::Database> {
    pub conditions: Vec<WhereClause<DB>>,
}

impl<DB: sqlx::Database> WhereGroup<DB> {
    pub fn new() -> Self {
        return Self {
            conditions: Vec::new()
        };
    }

    pub fn r#where<V>(&mut self, c: impl Into<String>, o: impl Into<String>, v: V) -> &mut Self 
    where
        V: Bindable<DB>,
    {
        self.conditions.push(WhereClause::Clause {
            connector: Connector::And,
            column: c.into(),
            operator: o.into(),
            value: Box::new(v),
        });
        self
    }

    pub fn and_where<V>(&mut self, c: impl Into<String>, o: impl Into<String>, v: V) -> &mut Self 
    where
        V: Bindable<DB>,
    {
        self.r#where(c, o, v)
    }

    pub fn or_where<V>(&mut self, c: impl Into<String>, o: impl Into<String>, v: V) -> &mut Self 
    where
        V: Bindable<DB>,
    {
        self.conditions.push(WhereClause::Clause {
            connector: Connector::Or,
            column: c.into(),
            operator: o.into(),
            value: Box::new(v),
        });
        self
    }
}


#[derive(Clone, Debug)]
pub struct Join {
    pub table: String,
    pub column: String,
    pub operator: String,
    pub column_table: String, 
    pub join_type: JoinType
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
            order: order
        }
    }
}

pub struct Limit<DB: sqlx::Database> {
    pub value: Box<dyn Bindable<DB>>,
}

impl <DB: sqlx::Database>Limit<DB> {
    pub fn new(limit: i64) -> Self
    where
        for<'i> i64: sqlx::Encode<'i, DB> + sqlx::Type<DB>,
    {
        Self { value: Box::new(limit) }
    }
}

pub struct Offset<DB: sqlx::Database> {
    pub value: Box<dyn Bindable<DB>>,
}

impl <DB: sqlx::Database>Offset<DB> {
    pub fn new(offset: i64) -> Self
    where
        for<'i> i64: sqlx::Encode<'i, DB> + sqlx::Type<DB>,
    {
        Self { value: Box::new(offset) }
    }
}

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
    pub values: HashMap<String, Box<dyn Bindable<DB>>>
}

impl <DB: sqlx::Database>Statement<DB> {
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
    pub items: Vec<Entity>
}