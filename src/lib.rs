#![feature(associated_type_defaults)]
#![feature(downcast_unchecked)]
#![feature(non_lifetime_binders)]

pub mod sqlite;
pub mod postgres;
pub mod mysql;
pub mod query;

use std::{any::Any, collections::HashMap, default, str, sync::LazyLock};

use anyhow::Result;
use sqlx::{FromRow, Pool, any::install_default_drivers};

use crate::query::{Order, Pagination, Statement};

pub(crate) static mut CONNECTIONS: LazyLock<HashMap<&str, Box<dyn std::any::Any>>> = LazyLock::new(|| HashMap::new());

pub struct DB;



pub struct Database {

}

impl Database {

    async fn get<O>(&self, statement: &Statement) -> Result<Vec<O>>
    where
        // O: for<'r> FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        todo!()
    }
}


impl DB {
    pub fn init() {
        install_default_drivers();
    }

    pub fn install() -> Result<()> {

        let a: u32;

        Ok(())
    }

    #[allow(static_mut_refs)]
    pub fn add(connection: &'static str, executor: impl Executor + 'static) {
        unsafe {
            // CONNECTIONS.insert(connection, Box::new(executor));
        }
    }

    #[allow(static_mut_refs)]
    pub fn remove(connection: &str) {
        unsafe {
            // CONNECTIONS.remove(connection);
        }
    }
}


#[allow(async_fn_in_trait)]
pub trait Executor {
    type T: sqlx::Database;

    async fn connect(url: &str) -> Self;

    fn table(&mut self, name: &str) -> &mut Self;

    fn select(&mut self, columns: Vec<&str>) -> &mut Self;

    fn order_by(&mut self, column: &str, order: Order) -> &mut Self;

    fn limit(&mut self, limit: u64) -> &mut Self;

    async fn first<O>(&self) -> Result<Vec<O>>
    where
        O: for<'r> FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized;

    async fn get<O>(&self, limit: u64) -> Result<Vec<O>>
    where
        O: for<'r> FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized;

    fn pagination<O>(&self, limit: u64, page: u64) -> Result<Pagination<O>>
    where
        O: for<'r> FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized;
}

