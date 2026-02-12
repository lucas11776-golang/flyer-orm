// #![feature(non_lifetime_binders)]

pub mod sqlite;
pub mod postgres;
pub mod mysql;
pub mod query;

use std::{collections::HashMap, str, sync::LazyLock};

use anyhow::Result;
use sqlx::{FromRow, any::{install_default_drivers, install_drivers}};

use crate::query::{Pagination, Query, Statement};


pub(crate) static mut CONNECTIONS: LazyLock<HashMap<&str, Box<dyn std::any::Any>>> = LazyLock::new(|| HashMap::new());



pub struct DB;


impl DB {
    pub fn init() {
        install_default_drivers();
    }

    pub fn install() -> Result<()> {
        Ok(())
    }

    #[allow(static_mut_refs)]
    pub fn add(connection: &'static str, executor: impl Executor + 'static) {
        unsafe {
            CONNECTIONS.insert(connection, Box::new(executor));
        }
    }

    #[allow(static_mut_refs)]
    pub fn remove(connection: &str) {
        unsafe {
            CONNECTIONS.remove(connection);
        }
    }


    #[allow(static_mut_refs)]
    pub fn query(connection: &str) -> Query {
        Query {
            statement: Statement::new(connection)
        }
    }
}

pub trait Executor: Send + Sync {
    fn get<T: sqlx::Database, O>(&self, statement: Statement) -> impl std::future::Future<Output = Result<Vec<O>>> + Send
    where
        O: for<'r> FromRow<'r,  <T as sqlx::Database>::Row> + Send + Unpin;

    fn pagination<T: sqlx::Database, O>(&self, statement: Statement) -> impl std::future::Future<Output = Result<Pagination<O>>> + Send
    where
        O: for<'r> FromRow<'r,  <T as sqlx::Database>::Row> + Send + Unpin;
}