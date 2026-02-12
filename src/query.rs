use std::ops::DerefMut;

use anyhow::Result;
use serde::Serialize;
use sqlx::{Database, FromRow, Transaction as SqlxTransaction};

use crate::{CONNECTIONS, Executor, mysql::MySQL, postgres::Postgres, sqlite::SQLite};

#[derive(Clone, Debug)]
pub struct Statement {
    pub(crate) connection: String,
    pub(crate) table: String,
    pub(crate) select: Vec<String>,
    pub(crate) where_clause: Vec<String>,
    pub(crate) join: Vec<String>,
    pub(crate) order_by: Option<(String, Order)>,
    pub(crate) limit: Option<u64>,
    pub(crate) offset: Option<u64>,
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
            offset: None,
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

pub struct Query<DB: sqlx::Database> {
    pub(crate) statement: Statement,
    pub(crate) database: DB
}

impl <DB: Database>Query<DB> {
    pub fn table(&mut self, name: &str) -> &mut Self {
        self.statement.table = name.to_owned();

        return self;
    }

    pub fn select(&mut self, columns: Vec<&str>) -> &mut Self {
        self.statement.select = columns.iter().map(|column| column.to_string()).collect();

        return self;
    }

    pub fn order_by(&mut self, column: &str, order: Order) -> &mut Self {
        self.statement.order_by = Some((column.to_owned(), order));

        return self;
    }

    pub fn limit(&mut self, limit: u64) -> &mut Self {
        self.statement.limit = Some(limit);

        return self;
    }

    pub fn close() {
    }

    // #[allow(static_mut_refs)]
    // pub fn get_executor<E: Executor>(&self, connection: &str) -> impl Executor
    // where
    //     // E: 

    // {
    //     todo!()
    // }
}



impl <DB: Database>Query<DB> {

    #[allow(static_mut_refs)]
    pub async fn all<O>(&self) -> Result<Vec<O>>
    where
        O: for<'r> FromRow<'r, <sqlx::Any as sqlx::Database>::Row> + Send + Unpin
    {

        
        todo!()
    }


    #[allow(static_mut_refs)]
    pub async fn get<O>(&self, limit: u64) -> Result<Vec<O>>
    where
        O: for<'r> FromRow<'r, <DB as sqlx::Database>::Row> + Send + Unpin
    {
        let mut stmt = self.statement.clone();

        stmt.limit = Some(limit);



        // TODO: find better way temp to make it work time being. 
        unsafe {
            let any = CONNECTIONS.get(self.statement.connection.as_str()).unwrap().as_ref();

            if let Some(exc) = any.downcast_ref::<MySQL>() {
                return exc.get::<DB, O>(self.statement.clone()).await;
            }

            if let Some(exc) = any.downcast_ref::<Postgres>() {
                return exc.get::<DB, O>(self.statement.clone()).await;
            }

            return any.downcast_ref::<SQLite>().unwrap().get::<DB, O>(self.statement.clone()).await;
        }
    }
    

    #[allow(static_mut_refs)]
    pub async fn first<O>(&self) -> Result<O>
    where
        O: for<'r> FromRow<'r, <sqlx::Any as sqlx::Database>::Row> + Send + Unpin
    {
        let mut stmt = self.statement.clone();

        stmt.limit = Some(1);


        todo!()
    }
    

    #[allow(static_mut_refs)]
    pub async fn pagination<O>(&self, limit: u64, page: u64) -> Result<Pagination<O>>
    where
        O: for<'r> FromRow<'r, <sqlx::Any as sqlx::Database>::Row> + Send + Unpin
    {
        let mut stmt = self.statement.clone();

        stmt.offset = None;
        stmt.limit = Some(limit);



        todo!()
    }
}