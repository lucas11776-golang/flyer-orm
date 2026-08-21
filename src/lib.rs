use std::sync::LazyLock;

use crate::connections::Connections;
use crate::query::scalar::Scalar;
use crate::query::{delete::Delete, insert::Insert, query::Query, raw::Raw, update::Update};
use crate::query::{Having, Join, Limit, Offset, OrderValue, Pagination, Statement};
use crate::types::{Bindable, QueryResult, WhereClause};

pub use database::mysql::MySQL;
pub use database::postgres::Postgres;
pub use database::sqlite::SQLite;
pub use executor::Executor;

pub use sqlx::MySqlPool;
pub use sqlx::PgPool;
use sqlx::Pool;
use sqlx::QueryBuilder;
pub use sqlx::SqlitePool;

pub mod database;
pub mod executor;
pub mod query;
pub mod types;

pub use anyhow::Result;
pub use sqlx;

mod connections;

pub trait Entity {}

pub use flyer_orm_derive::Entity;

static mut CONNECTIONS: LazyLock<Connections> = LazyLock::new(Connections::new);

pub struct Database<'q, E: Executor + 'static> {
    executor: &'q E,
}

impl <'q, E: Executor + 'static>Database<'q, E> {
    #[allow(static_mut_refs)]
    pub fn connection(connection: &str) -> Self {
       unsafe {
            Self {
                executor: CONNECTIONS.get::<E>(connection),
            }
       }
    }

    #[allow(static_mut_refs)]
    pub fn add_connection(connection: impl Into<String>, executor: E) {
        unsafe {
            CONNECTIONS.add(connection, executor);
        }
    }

    #[allow(static_mut_refs)]
    pub fn remove(connection: &str) {
        unsafe {
            CONNECTIONS.remove(connection);
        }
    }

    #[allow(static_mut_refs)]
    pub fn cache(path: &str) -> Result<&'static str> {
        unsafe {
            CONNECTIONS.cache(path)
        }
    }

    pub fn pool(&self) -> &Pool<E::DB> {
        self.executor.pool()
    }

    pub fn raw(&self, sql: &'q str) -> Raw<'q, E> {
        Raw::new(self.executor, Ok(sql))
    }

    pub fn raw_from_file(&self, path: &str) -> Raw<'q, E> {
        Raw::new(self.executor, Self::cache(path))
    }

    pub fn scalar(&self, sql: &'q str) -> Scalar<'q, E> {
        Scalar::new(self.executor, Ok(sql))
    }

    pub fn scalar_from_file(&self, path: &str) -> Scalar<'q, E> {
        Scalar::new(self.executor, Self::cache(path))
    }

    pub fn query(&self, table: &'q str) -> Query<'q, E> {
        Query::new(self.executor, table)
    }

    pub fn insert(&self, table: &'q str) -> Insert<'q, E> {
        Insert::new(self.executor, table)
    }

    pub fn update(&self, table: &'q str) -> Update<'q, E> {
        Update::new(self.executor, table)
    }

    pub fn delete(&self, table: &'q str) -> Delete<'q, E> {
        Delete::new(self.executor, table)
    }

    pub fn query_builder(&self, sql: &'q str) -> QueryBuilder<'q, E::DB> {
        QueryBuilder::new(sql)
    }
}