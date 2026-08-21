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

pub use anyhow::Result;
pub use sqlx;

mod connections;

pub trait Entity {}

pub use flyer_orm_derive::Entity;

static CONNECTIONS: LazyLock<Connections> = LazyLock::new(Connections::new);

pub struct Database<E: Executor + 'static> {
    executor: Arc<E>,
}

impl<E: Executor + 'static> Database<E> {
    pub fn connection(connection: &str) -> Self {
        Self {
            executor: CONNECTIONS.get::<E>(connection),
        }
    }

    pub fn add_connection(connection: impl Into<String>, executor: E) {
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