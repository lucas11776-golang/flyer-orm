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
use sqlx::Pool;
use sqlx::QueryBuilder;
pub use sqlx::SqlitePool;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::query::raw_from_file::RawFromFile;
use crate::query::scalar::Scalar;
use crate::query::{delete::Delete, insert::Insert, raw::Raw, update::Update, query::Query};
use crate::query::{Having, Join, Limit, Offset, OrderValue, Pagination, Statement};
use crate::types::{Bindable, QueryResult, WhereClause};

pub mod database;
pub mod query;
pub mod types;
pub mod executor;

pub use anyhow::Result;
pub use sqlx;

pub trait Entity {}

pub use flyer_orm_derive::Entity;

// static mut CONNECTIONS: LazyLock<HashMap<String, Arc<Box<dyn Any>>>> = LazyLock::new(|| HashMap::new());
static mut CONNECTIONS: LazyLock<Connections> = LazyLock::new(|| Connections::new());

pub(crate) struct Connections {
    cache: HashMap<String, String>,
    connections: HashMap<String, Arc<Box<dyn Any>>>
}

impl Connections {
    pub fn new() -> Self {
        Self {
            cache: Default::default(),
            connections: Default::default(),
        }
    }

    pub fn add<E: Executor + 'static>(&mut self, connection: impl Into<String>, executor: E) {
        self.connections.insert(connection.into(), Arc::new(Box::new(executor)));
    }

    #[allow(static_mut_refs)]
    pub fn get<'q, E: Executor + 'static>(&'q mut self, connection: impl Into<String>) -> &'q E {
        self
            .connections
            .get(&connection.into())
            .unwrap()
            .downcast_ref::<E>()
            .unwrap()
    }

    pub fn remove(&mut self, connection: impl Into<String>) {
    }

    pub async fn cache(&mut self, path: String) -> Result<String> {
        if let Some(sql) = self.cache.get(&path) {
            return Ok(sql.into())
        }

        let file = File::open(&path).await;

        if let Err(err) = file {
            return Err(err.into());
        }

        let mut cache = String::new();

        if let Err(err) = file.unwrap().read_to_string(&mut cache).await {
            return Err(err.into());
        }

        self.cache.insert(path, cache.clone());

        Ok(cache)
    }
}

pub struct Database<'q, E: Executor + 'static> {
    executor: &'q E,
}

impl <'q, E: Executor>Database<'q, E> {
    #[allow(static_mut_refs)]
    pub fn connection(connection: impl Into<String>) -> Self {
        unsafe {
            Self {
                executor: CONNECTIONS.get(connection)
            }
        }
    }

    #[allow(static_mut_refs)]
    pub fn add(connection: impl Into<String>, executor: E) {
        unsafe { CONNECTIONS.add(connection.into(), executor) }
    }
    
    #[allow(static_mut_refs)]
    pub fn remove(connection: impl Into<String>) {
        unsafe { CONNECTIONS.remove(&connection.into()) }
    }
        
    #[allow(static_mut_refs)]
    pub(crate) async fn cache(path: impl Into<String>) -> Result<String> {
        unsafe { CONNECTIONS.cache(path.into()).await }
    }

    pub fn pool(&self) -> &'q Pool<E::DB> {
        self
            .executor
            .pool()
    }

    pub fn raw(&self, sql: impl Into<String>) -> Raw<'q, E> {
        Raw::new(self.executor, sql)
    }

    pub fn raw_from_file(&self, path: impl Into<String>) -> RawFromFile<'q, E> {
        RawFromFile::new(self.executor, path)
    }

    pub fn scaler(&self, sql: &'q str) -> Scalar<'q, E>
    where
        E::DB: sqlx::Database,
    {
        Scalar::new(self.executor, sql)
    }

    pub fn scaler_from_file(&self, sql: &'q str) -> Scalar<'q, E>
    where
        E::DB: sqlx::Database,
    {
        Scalar::new(self.executor, sql)
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

    pub fn query_builder(&self) -> QueryBuilder<'q, E::DB> {
        QueryBuilder::default()
    }
}