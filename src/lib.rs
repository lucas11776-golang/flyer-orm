use std::sync::{Arc, LazyLock, RwLock};

use crate::connections::Connections;
use crate::query::scalar::Scalar;
use crate::query::{delete::Delete, insert::Insert, query::Query, raw::Raw, update::Update};

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

static CONNECTIONS: LazyLock<RwLock<Connections>> =
    LazyLock::new(|| RwLock::new(Connections::new()));

#[derive(Clone)]
pub struct Database<E: Executor + 'static> {
    executor: Arc<E>,
}

impl<E: Executor + 'static> Database<E> {
    #[inline]
    pub fn connection(connection: &str) -> Self {
        Self {
            executor: CONNECTIONS
                .read()
                .expect("CONNECTIONS lock poisoned")
                .get::<E>(connection),
        }
    }

    #[inline]
    pub fn add_connection(connection: impl Into<String>, executor: E) {
        CONNECTIONS
            .write()
            .expect("CONNECTIONS lock poisoned")
            .add(connection, executor);
    }

    #[inline]
    pub fn remove(connection: &str) {
        CONNECTIONS
            .write()
            .expect("CONNECTIONS lock poisoned")
            .remove(connection);
    }

    #[inline]
    pub fn cache(path: &str) -> Result<String> {
        CONNECTIONS
            .write()
            .expect("CONNECTIONS lock poisoned")
            .cache(path)
    }

    #[inline]
    pub fn pool(&self) -> &Pool<E::DB> {
        self.executor.pool()
    }

    #[inline]
    pub fn raw(&self, sql: impl Into<String>) -> Raw<E> {
        Raw::new(Arc::clone(&self.executor), Ok(sql.into()))
    }

    #[inline]
    pub fn raw_from_file(&self, path: &str) -> Raw<E> {
        Raw::new(Arc::clone(&self.executor), Self::cache(path))
    }

    #[inline]
    pub fn scalar(&self, sql: impl Into<String>) -> Scalar<E> {
        Scalar::new(Arc::clone(&self.executor), Ok(sql.into()))
    }

    #[inline]
    pub fn scalar_from_file(&self, path: &str) -> Scalar<E> {
        Scalar::new(Arc::clone(&self.executor), Self::cache(path))
    }

    #[inline]
    pub fn query(&self, table: impl Into<String>) -> Query<E> {
        Query::new(Arc::clone(&self.executor), table)
    }

    #[inline]
    pub fn insert(&self, table: impl Into<String>) -> Insert<E> {
        Insert::new(Arc::clone(&self.executor), table)
    }

    #[inline]
    pub fn update(&self, table: impl Into<String>) -> Update<E> {
        Update::new(Arc::clone(&self.executor), table)
    }

    #[inline]
    pub fn delete(&self, table: impl Into<String>) -> Delete<E> {
        Delete::new(Arc::clone(&self.executor), table)
    }

    #[inline]
    pub fn query_builder<'q>(&self, sql: &str) -> QueryBuilder<'q, E::DB> {
        QueryBuilder::new(sql)
    }
}
