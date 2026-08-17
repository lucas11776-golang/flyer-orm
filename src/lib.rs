use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::LazyLock;

pub use database::mysql::MySQL;
pub use database::postgres::Postgres;
pub use database::sqlite::SQLite;
pub use executor::Executor;

pub use sqlx::MySqlPool;
pub use sqlx::PgPool;
pub use sqlx::SqlitePool;

use crate::query::delete::Delete;
use crate::query::query::Query;
use crate::query::update::Update;
use crate::{
    query::{Having, Join, Limit, Offset, OrderValue, Pagination, Statement, insert::Insert, raw::Raw},
    types::{Bindable, QueryResult, WhereClause},
};

pub mod database;
pub mod query;
pub mod types;
pub mod executor;

pub use anyhow::Result;
pub use sqlx;

pub trait Entity {}

pub use flyer_orm_derive::Entity;

static mut CONNECTIONS: LazyLock<HashMap<String, Arc<Box<dyn Any>>>> = LazyLock::new(|| HashMap::new());

pub struct Database<'q, E: Executor + 'static> {
    executor: &'q E,
}

impl <'q, E: Executor>Database<'q, E> {
    #[allow(static_mut_refs)]
    pub fn connection(connection: impl Into<String>) -> Self {
        unsafe {
            Self {
                executor: CONNECTIONS
                    .get(&connection.into())
                    .unwrap()
                    .downcast_ref::<E>()
                    .unwrap()
            }
        }
    }

    #[allow(static_mut_refs)]
    pub fn add(connection: impl Into<String>, executor: E) {
        unsafe {
            CONNECTIONS.insert(connection.into(), Arc::new(Box::new(executor)));
        }
    }
    
    #[allow(static_mut_refs)]
    pub fn remove(connection: impl Into<String>) {
        unsafe {
            CONNECTIONS.remove(&connection.into());
        }
    }

    pub fn raw(&self, sql: impl Into<String>) -> Raw<'q, E> {
        Raw::new(self.executor, sql)
    }

    pub fn query(&self, table: impl Into<String>) -> Query<'q, E> {
        Query::new(self.executor, table)
    }

    pub fn insert(&self, table: impl Into<String>) -> Insert<'q, E> {
        Insert::new(self.executor, table)
    }

    pub fn update(&self, table: impl Into<String>) -> Update<'q, E> {
        Update::new(self.executor, table)
    }

    pub fn delete(&self, table: impl Into<String>) -> Delete<'q, E> {
        Delete::new(self.executor, table)
    }
}