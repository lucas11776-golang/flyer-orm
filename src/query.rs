use anyhow::Result;
use serde::Serialize;
use sqlx::{FromRow, Transaction as SqlxTransaction};

#[derive(Clone, Debug)]
pub struct Statement {
    pub(crate) connection: String,
    pub(crate) table: String,
    pub(crate) select: Vec<String>,
    pub(crate) where_clause: Vec<String>,
    pub(crate) join: Vec<String>,
    pub(crate) order_by: Option<Order>,
    pub(crate) limit: Option<u64>,
}

impl Statement {
    pub(crate) fn new(connection: &str) -> Self {
        return Self {
            connection: connection.to_owned(),
            table: String::new(),
            select: Vec::new(),
            where_clause: Vec::new(),
            join: Vec::new(),
            order_by: None,
            limit: None,
        }
    }
}

pub struct Transaction<'t, T: sqlx::Database> {
    transaction: SqlxTransaction<'t, T>
}

impl <'t, T: sqlx::Database>Transaction<'t, T> {
    fn new(transaction: SqlxTransaction<'t, T>) -> Self {
        return Self {
            transaction: transaction
        }
    }
}

#[derive(Serialize)]
pub struct Pagination<Entity> {
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
    pub items: Vec<Entity>
}


#[derive(Clone, Debug)]
pub enum Order {
    ASC,
    DESC
}

pub struct Query {
    pub(crate) statement: Statement,
}

impl Query {
    pub fn table(&self, name: &str) -> &Self {
        return self;
    }

    pub fn select(&self, columns: Vec<&str>) -> &Self {
        return self;
    }

    pub fn order_by(&self, column: &str, order: Order) -> &Self {
        return self;
    }

    pub fn limit(&self, limit: u64) -> &Self {
        return self;
    }
}

impl Query {
    pub async fn all<O>(&self) -> Result<Vec<O>>
    where
        O: for<'r> FromRow<'r, <sqlx::Any as sqlx::Database>::Row> + Send + Unpin
    {
        todo!()
    }

    pub async fn get<O>(&self, limit: u64) -> Result<Vec<O>>
    where
        O: for<'r> FromRow<'r, <sqlx::Any as sqlx::Database>::Row> + Send + Unpin
    {
        todo!()
    }
    
    pub async fn first<O>(&self) -> Result<O>
    where
        O: for<'r> FromRow<'r, <sqlx::Any as sqlx::Database>::Row> + Send + Unpin {
        todo!()
    }
    
    pub async fn pagination<O>(&self, limit: u64, page: u64) -> Result<Pagination<O>>
    where
        O: for<'r> FromRow<'r, <sqlx::Any as sqlx::Database>::Row> + Send + Unpin {
        todo!()
    }
}