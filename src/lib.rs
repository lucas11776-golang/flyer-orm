pub mod sqlite;
pub mod postgres;
pub mod mysql;
pub mod query;

use std::{collections::HashMap, marker::PhantomData, str, sync::LazyLock};

use anyhow::{Ok, Result};
use sqlx::{FromRow, Pool, any::install_default_drivers};

use crate::query::{Order, Pagination, Statement};

pub(crate) static mut CONNECTIONS: LazyLock<HashMap<&str, String>> = LazyLock::new(|| HashMap::new());

pub struct DB;

impl DB {
    pub fn init() {
        install_default_drivers();
    }

    #[allow(static_mut_refs)]
    pub fn add(connection: &'static str, url: &str) {
        unsafe { CONNECTIONS.insert(connection, url.to_string()); }
    }

    #[allow(static_mut_refs)]
    pub fn remove(connection: &str) {
        unsafe { CONNECTIONS.remove(connection); }
    }

    #[allow(static_mut_refs)]
    pub fn query<Exc>(connection: &str) -> Query::<Exc>
    where
        Exc: Executor
    {
        return unsafe { Query::new(CONNECTIONS.get(connection).unwrap()) };
    }
}

pub struct Query<Exc> {
    statement: Statement,
    _marker: PhantomData<Exc>
}

impl <Exc>Query<Exc>
where
    Exc: Executor
{
    pub fn new(url: &str) -> Self {
        return Self {
            statement: Statement::new(url),
            _marker: PhantomData
        }
    }

    pub fn table(&mut self, name: &str) -> &mut Self {
        self.statement.table = name.to_string();

        return self;
    }

    pub fn select(&mut self, _columns: Vec<&str>) -> &mut Self {
        return self;
    }

    pub fn order_by(&mut self, _column: &str, _order: Order) -> &mut Self {
        return self;
    }

    pub fn limit(&mut self, _limit: u64) -> &mut Self {
        return self;
    }

    pub async fn get<O>(&mut self, limit: u64) -> Result<Vec<O>>
    where
        O: for<'r> FromRow<'r, <Exc::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        self.statement.limit = Some(limit);

        return Ok(Exc::default().get(&self.statement).await.unwrap());
    }

    async fn paginate<O>(&mut self, limit: u64, page: u64) -> Result<Pagination<O>>
    where
        O: for<'r> FromRow<'r, <Exc::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        self.statement.limit = Some(limit);
        self.statement.offset = Some(page); // TODO: calc offset

        return Ok(Exc::default().paginate(&self.statement).await.unwrap());
    }
}

#[allow(async_fn_in_trait)]
pub trait Executor: Default {
    type T: sqlx::Database;

    async fn db(&self, url: &str) -> Result<Pool<Self::T>>; 

    async fn get<O>(&self, statement: &Statement) -> Result<Vec<O>>
    where
        O: for<'r> FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized;

    async fn paginate<O>(&self, statement: &Statement) -> Result<Pagination<O>>
    where
        O: for<'r> FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized;
}