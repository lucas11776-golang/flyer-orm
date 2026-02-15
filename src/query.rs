

use std::{any::Any, mem, ops::DerefMut};

use anyhow::Result;
use serde::Serialize;
use sqlx::{Database, FromRow, Row, Sqlite, Transaction as SqlxTransaction, any::AnyRow};

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
    // pub(crate) fn new(connection: &str) -> Self {
    pub(crate) fn new(connection: &str) -> Self {
        return Self {
            // connection: connection.to_owned(),
            connection: String::new(),
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
