pub mod sqlite;
pub mod postgres;
pub mod mysql;
pub mod query;

use std::{collections::HashMap, marker::PhantomData, str, sync::LazyLock};

use anyhow::{Result};
use sqlx::{Arguments, Encode, FromRow, Pool, any::install_default_drivers};
use sqlx::types::Type;

use crate::query::{JoinQuery, Order, OrderQuery, Pagination, QueryStatement, Statement, WhereQuery};

pub(crate) static mut CONNECTIONS: LazyLock<HashMap<&str, String>> = LazyLock::new(|| HashMap::new());

#[allow(async_fn_in_trait)]
pub trait Executor: Default {
    type T: sqlx::Database;

    async fn db<'q>(&self, url: &str) -> Result<Pool<Self::T>>; 

    fn to_sql<'q>(&'q self, statement: &'q Statement<'q, Self::T>) -> Result<String>;

    async fn first<'q, O>(&self, statement: &'q Statement<'q, Self::T>) -> Result<O>
    where
        O: for<'r> FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized;

    async fn get<'q, O>(&self, statement: &'q Statement<'q, Self::T>) -> Result<Vec<O>>
    where
        O: for<'r> FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized;

    async fn paginate<'q, O>(&self, statement: &'q Statement<'q, Self::T>) -> Result<Pagination<O>>
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
        Exc: Executor
    {
        return unsafe { Query::new(CONNECTIONS.get(connection).unwrap()) };
    }

    #[allow(static_mut_refs)]
    pub fn query_url<'q, Exc>(url: &'q str) -> Query::<'q, Exc>
    where
        Exc: Executor
    {
        return Query::new(url);
    }
}


pub struct Query<'q, Exc: Executor> {
    statement: Statement<'q, Exc::T>,
    _marker: PhantomData<Exc>
}

impl <'q, Exc>Query<'q, Exc>
where
    Exc: Executor
{
    pub fn new(url: &str) -> Self {
        return Self {
            statement: Statement::<'q, Exc::T>::new(url),
            _marker: PhantomData
        }
    }

    pub fn table(&mut self, name: &'q str) -> &mut Self {
        self.statement.query.table = name.to_string();

        return self;
    }

    pub fn select(&mut self, columns: Vec<&str>) -> &mut Self {
        self.statement.query.select = columns.iter().map(|c| c.to_string()).collect();

        return self;
    }

    // TODO: find better name...
    pub fn r#where<T: 'q + Encode<'q, Exc::T> + Type<Exc::T>>(&mut self, column: &str, operator: &str, val: T) -> &mut Self {
        self.statement.query.where_queries.push(WhereQuery {
            column: column.to_string(),
            operator: operator.to_string(),
            position: None // TODO: find better way for position...
        });

        self.statement.arguments.add(val).unwrap();
        
        return self;
    }

    pub fn and_where<T: 'q + Encode<'q, Exc::T> + Type<Exc::T>>(&mut self, column: &str, operator: &str, val: T) -> &mut Self {
        self.statement.query.where_queries.push(WhereQuery {
            column: column.to_string(),
            operator: operator.to_string(),
            position: Some(query::WhereQueryPosition::AND) // TODO: find better way for position...
        });

        self.statement.arguments.add(val).unwrap();
        
        return self;
    }

    pub fn or_where<T: 'q + Encode<'q, Exc::T> + Type<Exc::T>>(&mut self, column: &str, operator: &str, val: T) -> &mut Self {
        self.statement.query.where_queries.push(WhereQuery {
            column: column.to_string(),
            operator: operator.to_string(),
            position: Some(query::WhereQueryPosition::OR) // TODO: find better way for position...
        });

        self.statement.arguments.add(val).unwrap();

        return self;
    }

    pub fn order_by(&mut self, column: &str, order: Order) -> &mut Self {
        self.statement.query.order_by.push(OrderQuery {
            column: column.to_string(),
            order: order
        });

        return self;
    }

    pub fn join(&mut self, table: &str, column: &str, operator: &str, column_table: &str) -> &mut Self {
        self.statement.query.join.push(JoinQuery {
            table: table.to_string(),
            column: column.to_string(),
            operator: operator.to_string(),
            column_table: column_table.to_string(),
        });

        return self;
    }

    pub fn limit(&mut self, limit: u64) -> &mut Self {
        self.statement.query.limit = Some(limit);

        return self;
    }

    pub async fn get<O>(&'q mut self, limit: u64) -> Result<Vec<O>>
    where
        O: for<'r> FromRow<'r, <Exc::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        self.statement.query.limit = Some(limit);

        return Ok(Exc::default().get::<O>(&self.statement).await.unwrap());
    }

    pub async fn paginate<O>(&'q mut self, limit: u64, page: u64) -> Result<Pagination<O>>
    where
        O: for<'r> FromRow<'r, <Exc::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        self.statement.query.limit = Some(limit);
        self.statement.query.page = Some(page); // TODO: calc offset

        return Ok(Exc::default().paginate::<O>(&self.statement).await.unwrap());
    }
}


pub(crate) trait QueryBuilder {
    fn build<'q>(statement: &'q QueryStatement) -> Result<String>;
}