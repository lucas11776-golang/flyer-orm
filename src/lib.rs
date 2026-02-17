pub mod sqlite;
pub mod postgres;
pub mod mysql;
pub mod query;

use std::{collections::HashMap, marker::PhantomData, str, sync::LazyLock};

use anyhow::{Result};
use sqlx::{Arguments, Encode, FromRow, Pool, any::install_default_drivers};
use sqlx::types::Type;

use crate::query::{JoinQuery, Order, OrderQuery, Pagination, Statement, WhereQuery};

pub(crate) static mut CONNECTIONS: LazyLock<HashMap<&str, String>> = LazyLock::new(|| HashMap::new());

#[allow(async_fn_in_trait)]
pub trait Executor<'q> {
    type T: sqlx::Database;

    fn new(statement: &'q Statement<'q, Self::T>) -> Self where Self: Sized;

    async fn db(&self, url: &str) -> Result<Pool<Self::T>>; 

    fn to_sql(&'q self) -> Result<String>;

    async fn first<O>(&'q self) -> Result<O>
    where
        O: for<'r> FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized;

    async fn get<O>(& self) -> Result<Vec<O>>
    where
        O: for<'r> FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized;

    async fn paginate<O>(&mut self) -> Result<Pagination<O>>
    where
        O: for<'r> FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized;
}

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
    pub fn query<'q, Exc>(connection: &str) -> Query::<'q, Exc>
    where
        Exc: Executor<'q>
    {
        return unsafe { Query::new(CONNECTIONS.get(connection).unwrap()) };
    }

    #[allow(static_mut_refs)]
    pub fn query_url<'q, Exc>(url: &'q str) -> Query::<'q, Exc>
    where
        Exc: Executor<'q>
    {
        return Query::new(url);
    }
}


pub struct Query<'q, Exc: Executor<'q>> {
    statement: Statement<'q, Exc::T>,
    _marker: PhantomData<Exc>
}

impl <'q, Exc>Query<'q, Exc>
where
    Exc: Executor<'q>
{
    pub fn new(url: &str) -> Self {
        return Self {
            statement: Statement::<'q, Exc::T>::new(url),
            _marker: PhantomData
        }
    }

    pub fn table(&mut self, name: &'q str) -> &mut Self {
        self.statement.table = name.to_string();

        return self;
    }

    pub fn select(&mut self, columns: Vec<&str>) -> &mut Self {
        self.statement.select = columns.iter().map(|c| c.to_string()).collect();

        return self;
    }

    // TODO: find better name...
    pub fn r#where<T: 'q + Encode<'q, Exc::T> + Type<Exc::T>>(&mut self, column: &str, operator: &str, val: T) -> &mut Self {
        self.statement.where_queries.push(WhereQuery {
            column: column.to_string(),
            operator: operator.to_string(),
            position: None // TODO: find better way for position...
        });

        self.statement.arguments.add(val).unwrap();
        
        return self;
    }

    pub fn and_where<T: 'q + Encode<'q, Exc::T> + Type<Exc::T>>(&mut self, column: &str, operator: &str, val: T) -> &mut Self {
        self.statement.where_queries.push(WhereQuery {
            column: column.to_string(),
            operator: operator.to_string(),
            position: Some(query::WhereQueryPosition::AND) // TODO: find better way for position...
        });

        self.statement.arguments.add(val).unwrap();
        
        return self;
    }

    pub fn or_where<T: 'q + Encode<'q, Exc::T> + Type<Exc::T>>(&mut self, column: &str, operator: &str, val: T) -> &mut Self {
        self.statement.where_queries.push(WhereQuery {
            column: column.to_string(),
            operator: operator.to_string(),
            position: Some(query::WhereQueryPosition::OR) // TODO: find better way for position...
        });

        self.statement.arguments.add(val).unwrap();

        return self;
    }

    pub fn order_by(&mut self, column: &str, order: Order) -> &mut Self {
        self.statement.order_by.push(OrderQuery {
            column: column.to_string(),
            order: order
        });

        return self;
    }

    pub fn join(&mut self, table: &str, column: &str, operator: &str, column_table: &str) -> &mut Self {
        self.statement.join.push(JoinQuery {
            table: table.to_string(),
            column: column.to_string(),
            operator: operator.to_string(),
            column_table: column_table.to_string(),
        });

        return self;
    }

    pub fn limit(&mut self, limit: u64) -> &mut Self {
        self.statement.limit = Some(limit);

        return self;
    }

    pub async fn get<O>(&'q mut self, limit: u64) -> Result<Vec<O>>
    where
        O: for<'r> FromRow<'r, <Exc::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        self.statement.limit = Some(limit);

        return Ok(Exc::new(&self.statement).get::<O>().await.unwrap());
    }

    pub async fn paginate<O>(&'q mut self, limit: u64, page: u64) -> Result<Pagination<O>>
    where
        O: for<'r> FromRow<'r, <Exc::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        self.statement.limit = Some(limit);
        self.statement.page = Some(page); // TODO: calc offset

        return Ok(Exc::new(&mut self.statement).paginate::<O>().await.unwrap());
    }
}


pub(crate) trait QueryBuilder<DB: sqlx::Database> {
    // fn new() -> Self where Self: Sized; 
    fn build<'q>(statement: &'q Statement<'q, DB>) -> Result<String>;
}