use anyhow::Result;
use serde::Serialize;

use crate::query::logic::SqlQuery;

pub mod logic;

pub(crate) trait QueryBuilder<'q> {
    fn new(statement: &'q SqlQuery) -> Self where Self: Sized;
    fn query(&self) -> Result<String>;
    fn insert(&self) -> Result<String>;
    fn update(&self) -> Result<String>;
    fn delete(&self) -> Result<String>;
    fn select(&self) -> Result<String>;
    fn from(&self) -> Result<String>;
    fn join(&self) -> Result<String>;
    fn r#where(&self) -> Result<String>;
    fn group_by(&self) -> Result<String>;
    fn having(&self) -> Result<String>;
    fn order_by(&self) -> Result<String>;
    fn limit(&self) -> Result<String>;
}

pub trait QueryResult {
    fn rows_affected(&self) -> u64;
    fn last_inserted(&self) -> u64;
}

#[derive(Clone, Debug)]
pub enum Order {
    ASC,
    DESC
}

impl ToString for Order {
    fn to_string(&self) -> String {
        return match self {
            Order::ASC => String::from("ASC"),
            Order::DESC => String::from("DESC"),
        };
    }
}

#[derive(Clone, Debug)]
pub struct Statement<'q, DB: sqlx::Database> {
    pub query: SqlQuery,
    pub arguments: DB::Arguments<'q>, 
}

#[derive(Debug, sqlx::FromRow)]
pub(crate) struct Total {
    pub total: u64
}

impl <'q, DB>Statement<'q, DB>
where
    DB: sqlx::Database
{
    pub(crate) fn new(table: &str) -> Self {
        return Self {
            query: SqlQuery::new(table),
            arguments: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct Transaction<'t, T: sqlx::Database> {
    transaction: sqlx::Transaction<'t, T>
}

impl <'t, T: sqlx::Database>Transaction<'t, T> {
    pub(crate) fn new(transaction: sqlx::Transaction<'t, T>) -> Self {
        return Self { transaction: transaction }
    }

    pub async fn commit(self) -> Result<()> {
        return self.transaction.commit().await.map_err(|e| e.into());
    }

    pub async fn rollback(self) -> Result<()> {
        return self.transaction.rollback().await.map_err(|e| e.into());
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct Pagination<Entity> {
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
    pub items: Vec<Entity>
}
