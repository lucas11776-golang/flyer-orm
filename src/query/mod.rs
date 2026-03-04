use anyhow::Result;
use serde::Serialize;
use sqlx::{Transaction as SqlxTransaction};

use crate::query::logic::{Condition, Join, OrderQuery, Where};

pub mod logic;

pub(crate) trait QueryBuilder<'q> {
    fn new(statement: &'q Query) -> Self where Self: Sized;
    fn query(&self) -> Result<String>;
    fn insert(&self) -> Result<String>;
    fn update(&self) -> Result<String>;
    fn delete(&self) -> Result<String>;
    fn select(&self) -> Result<String>;
    fn join(&self) -> Result<String>;
    fn r#where(&self) -> Result<String>;
    fn group_by(&self) -> Result<String>;
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
    pub query: Query,
    pub arguments: DB::Arguments<'q>, 
}

#[derive(Clone, Debug, Default)]
pub struct Having {
    pub column: String,
    pub operator: String,
    pub value: String,
    pub position: Option<Condition>
}

#[derive(Clone, Debug, Default)]
pub struct Query {
    pub table: String,
    pub select: Vec<String>,
    pub join: Vec<Join>,
    pub where_queries: Vec<Where>,
    pub group_by: Option<String>,
    pub having: Vec<Having>,
    pub order_by: Vec<OrderQuery>,
    pub limit: Option<u64>,
    pub page: Option<u64>, // TODO: must use `offset` or `page` must decide...
    pub columns: Option<Vec<String>>,
}

impl Query {
    pub fn new(table: &str) -> Self {
        return Self {
            table: table.to_string(),
            select: Vec::new(),
            join: Vec::new(),
            where_queries: Vec::new(),
            having: Vec::new(),
            group_by: None,
            order_by: Vec::new(),
            limit: None,
            page: None,
            columns: None,
        }
    }
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
            query: Query::new(table),
            arguments: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct Transaction<'t, T: sqlx::Database> {
    transaction: SqlxTransaction<'t, T>
}

impl <'t, T: sqlx::Database>Transaction<'t, T> {
    pub(crate) fn new(transaction: SqlxTransaction<'t, T>) -> Self {
        return Self {
            transaction: transaction
        }
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
