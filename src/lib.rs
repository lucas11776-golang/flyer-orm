use std::any::Any;
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::sync::Arc;
use std::sync::LazyLock;

use crate::query::scalar::Scalar;
use crate::query::{delete::Delete, insert::Insert, raw::Raw, update::Update, query::Query};
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
pub mod query;
pub mod types;
pub mod executor;

pub use anyhow::Result;
pub use sqlx;

pub trait Entity {}

pub use flyer_orm_derive::Entity;

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
        self.connections.remove(&connection.into());
    }

    // TODO: improve caching.
    pub fn cache(&mut self, path: String) -> Result<&str> {
        if self.cache.get(&path).is_some() {
            return Ok(self.cache.get(&path).unwrap());
        }

        let file = fs::File::open(&path);

        if let Err(err) = file {
            return Err(err.into());
        }

        let mut cache = String::new();

        if let Err(err) = file.unwrap().read_to_string(&mut cache) {
            return Err(err.into());
        }

        self.cache.insert(path.clone(), cache.clone());

        Ok(self.cache.get(&path).unwrap())
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
    pub(crate) fn cache<'a>(path: impl Into<String>) -> Result<&'a str> {
        unsafe { CONNECTIONS.cache(path.into()) }
    }

    pub fn pool(&self) -> &'q Pool<E::DB> {
        self
            .executor
            .pool()
    }

    pub fn raw(self, sql: &'q str) -> Raw<'q, E> {
        Raw::new(self.executor, Ok(sql))
    }

    pub fn raw_from_file(self, path: impl Into<String>) -> Raw<'q, E> {
        Raw::new(self.executor, Database::<E>::cache(path))
    }

    pub fn scaler(self, sql: &'q str) -> Scalar<'q, E> {
        Scalar::new(self.executor, Ok(sql))
    }

    pub fn scaler_from_file(self, path: &'q str) -> Scalar<'q, E> {
        Scalar::new(self.executor, Database::<E>::cache(path))
    }

    pub fn query(self, table: &'q str) -> Query<'q, E> {
        Query::new(self.executor, table)
    }

    pub fn insert(self, table: &'q str) -> Insert<'q, E> {
        Insert::new(self.executor, table)
    }

    pub fn update(self, table: &'q str) -> Update<'q, E> {
        Update::new(self.executor, table)
    }

    pub fn delete(self, table: &'q str) -> Delete<'q, E> {
        Delete::new(self.executor, table)
    }

    pub fn query_builder(self) -> QueryBuilder<'q, E::DB> {
        QueryBuilder::default()
    }
}