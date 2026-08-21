use std::sync::{Arc, LazyLock};

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
pub mod utils;

pub use anyhow::Result;
pub use sqlx;

mod connections;

pub trait Entity {}

pub use flyer_orm_derive::Entity;

static mut CONNECTIONS: LazyLock<Connections> = LazyLock::new(Connections::new);

pub struct Database<E: Executor + 'static> {
    executor: Arc<E>,
}

impl <E: Executor + 'static>Database<E> {
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
    pub fn cache(path: impl Into<String>) -> Result<String> {
        unsafe {
            CONNECTIONS.cache(&path.into())
        }
    }

    pub fn pool(&self) -> &Pool<E::DB> {
        self.executor.pool()
    }

    pub fn raw(&self, sql: impl Into<String>) -> Raw<E> {
        Raw::new(self.executor.clone(), Ok(sql.into()))
    }

    pub fn raw_from_file(&self, path: impl Into<String>) -> Raw<E> {
        Raw::new(self.executor.clone(), Self::cache(path))
    }

    pub fn scalar(&self, sql: impl Into<String>) -> Scalar<E> {
        Scalar::new(self.executor.clone(), Ok(sql.into()))
    }

    pub fn scalar_from_file(&self, path: impl Into<String>) -> Scalar<E> {
        Scalar::new(self.executor.clone(), Self::cache(path))
    }

    pub fn query(&self, table: impl Into<String>) -> Query<E> {
        Query::new(self.executor.clone(), table)
    }

    pub fn insert(&self, table: impl Into<String>) -> Insert<E> {
        Insert::new(self.executor.clone(), table)
    }

    pub fn update(&self, table: impl Into<String>) -> Update<E> {
        Update::new(self.executor.clone(), table)
    }

    pub fn delete(&self, table: impl Into<String>) -> Delete<E> {
        Delete::new(self.executor.clone(), table)
    }

    pub fn query_builder<'q>(&self, sql: impl Into<String>) -> QueryBuilder<'q, E::DB> {
        QueryBuilder::new(sql)
    }
}