use std::any::Any;
use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, LazyLock, RwLock};

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

pub trait Entity {}

pub use flyer_orm_derive::Entity;

static CONNECTIONS: LazyLock<Connections> = LazyLock::new(Connections::new);

pub(crate) struct Connections {
    cache: RwLock<HashMap<String, &'static str>>,
    connections: RwLock<HashMap<String, Arc<dyn Any + Send + Sync>>>,
}

impl Connections {
    pub fn new() -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
            connections: RwLock::new(HashMap::new()),
        }
    }

    pub fn add<E: Executor + 'static>(&self, connection: impl Into<String>, executor: E) {
        self.connections
            .write()
            .unwrap()
            .insert(connection.into(), Arc::new(executor));
    }

    pub fn get<E: Executor + 'static>(&self, connection: &str) -> Arc<E> {
        self.connections
            .read()
            .unwrap()
            .get(connection)
            .cloned()
            .and_then(|any| any.downcast::<E>().ok())
            .unwrap_or_else(|| panic!("Connection '{connection}' not found or type mismatch"))
    }

    pub fn remove(&self, connection: &str) {
        self.connections.write().unwrap().remove(connection);
    }

    pub fn cache(&self, path: &str) -> Result<&'static str> {
        if let Some(&cached) = self.cache.read().unwrap().get(path) {
            return Ok(cached);
        }

        let content = fs::read_to_string(path)?;
        let static_str: &'static str = Box::leak(content.into_boxed_str());

        let mut cache = self.cache.write().unwrap();

        Ok(*cache.entry(path.to_string()).or_insert(static_str))
    }
}

pub struct Database<E: Executor + 'static> {
    executor: Arc<E>,
}

impl<E: Executor + 'static> Database<E> {
    pub fn connection(connection: &str) -> Self {
        Self {
            executor: CONNECTIONS.get::<E>(connection),
        }
    }

    pub fn add(connection: impl Into<String>, executor: E) {
        CONNECTIONS.add(connection, executor);
    }

    pub fn remove(connection: &str) {
        CONNECTIONS.remove(connection);
    }

    pub fn cache(path: &str) -> Result<&'static str> {
        CONNECTIONS.cache(path)
    }

    pub fn pool(&self) -> &Pool<E::DB> {
        self.executor.pool()
    }

    pub fn raw<'q>(&'q self, sql: &'q str) -> Raw<'q, E> {
        Raw::new(self.executor.as_ref(), Ok(sql))
    }

    pub fn raw_from_file<'q>(&'q self, path: &str) -> Raw<'q, E> {
        Raw::new(self.executor.as_ref(), Self::cache(path))
    }

    pub fn scalar<'q>(&'q self, sql: &'q str) -> Scalar<'q, E> {
        Scalar::new(self.executor.as_ref(), Ok(sql))
    }

    pub fn scalar_from_file<'q>(&'q self, path: &str) -> Scalar<'q, E> {
        Scalar::new(self.executor.as_ref(), Self::cache(path))
    }

    pub fn query<'q>(&'q self, table: &'q str) -> Query<'q, E> {
        Query::new(self.executor.as_ref(), table)
    }

    pub fn insert<'q>(&'q self, table: &'q str) -> Insert<'q, E> {
        Insert::new(self.executor.as_ref(), table)
    }

    pub fn update<'q>(&'q self, table: &'q str) -> Update<'q, E> {
        Update::new(self.executor.as_ref(), table)
    }

    pub fn delete<'q>(&'q self, table: &'q str) -> Delete<'q, E> {
        Delete::new(self.executor.as_ref(), table)
    }

    pub fn query_builder<'q>(&'q self) -> QueryBuilder<'q, E::DB> {
        QueryBuilder::default()
    }
}