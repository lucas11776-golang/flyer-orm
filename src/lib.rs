pub mod sqlite;
pub mod postgres;
pub mod mysql;
pub mod query;

use std::{collections::HashMap, marker::PhantomData, str, sync::LazyLock};

use anyhow::{Ok, Result};
use sqlx::{Arguments, Encode, FromRow, Pool, types::Type};

use crate::query::{JoinQuery, JoinType, Order, OrderQuery, Pagination, QueryStatement, Statement, Transaction, WhereQuery, WhereQueryGroup};

pub(crate) static mut CONNECTIONS: LazyLock<HashMap<&str, String>> = LazyLock::new(|| HashMap::new());

#[allow(async_fn_in_trait)]
pub trait Executor {
    type T: sqlx::Database;

    async fn new(url: &str) -> Self where Self: Sized;

    fn db<'q>(&'q self) -> &'q Pool<Self::T>; 

    fn to_sql<'q>(&self, statement: &'q Statement<'q, Self::T>) -> Result<String>;

    async fn insert<'q>(&self, statement: &'q Statement<'q, Self::T>) -> Result<bool>;

    async fn insert_as<'q, O>(&self, statement: &'q Statement<'q, Self::T>) -> Result<O>
    where
        O: for<'r> FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized;

    async fn query_all<'q, O, T: 'q + Encode<'q, Self::T> + Type<Self::T>>(&self, sql: &str, args: Vec<T>) -> Result<Vec<O>>
    where
        O: for<'r> FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized;

    async fn query_one<'q, O, T: 'q + Encode<'q, Self::T> + Type<Self::T>>(&self, sql: &str, args: Vec<T>) -> Result<O>
    where
        O: for<'r> FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized;

    async fn all<'q, O>(&self, statement: &'q Statement<'q, Self::T>) -> Result<Vec<O>>
    where
        O: for<'r> FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized;

    async fn first<'q, O>(&self, statement: &'q Statement<'q, Self::T>) -> Result<O>
    where
        O: for<'r> FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized;

    async fn paginate<'q, O>(&self, statement: &'q Statement<'q, Self::T>) -> Result<Pagination<O>>
    where
        O: for<'r> FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized;
}

pub struct DB;

impl DB {
    #[allow(static_mut_refs)]
    pub fn add(connection: &'static str, url: &str) {
        unsafe { CONNECTIONS.insert(connection, url.to_string()); }
    }

    #[allow(static_mut_refs)]
    pub fn remove(connection: &str) {
        unsafe { CONNECTIONS.remove(connection); }
    }


    #[allow(static_mut_refs)]
    pub async fn db<E: Executor>(connection: &str) -> Database::<E>
    {
        return unsafe { Database::new(CONNECTIONS.get(connection).unwrap()).await };
    }
}

#[derive(Debug)]
pub struct Database<E: Executor> {
    executor: E,
}

impl <E: Executor>Database<E> {
    pub async fn new(url: &str) -> Self {
        return Self {
            executor: E::new(url).await,
        }
    }

    pub async fn transaction<'q>(&self) -> Result<Transaction<'q, E::T>> {
        return Ok(Transaction::new(self.executor.db().begin().await.unwrap()));
    }

    pub fn query<'q>(&'q self, table: &str) -> Query<'q, E> {
        return Query::new(table, &self.executor);
    }

    pub async fn close(&self) -> Result<()> {
        return Ok(self.executor.db().close().await);
    }
}

pub struct Query<'q, E: Executor> {
    db: &'q E,
    statement: Statement<'q, E::T>,
    _marker: PhantomData<E>
}

impl <'q, E>Query<'q, E>
where
    E: Executor
{
    pub fn new(table: &str, exc: &'q E) -> Self {
        return Self {
            db: exc,
            statement: Statement::<'q, E::T>::new(table),
            _marker: PhantomData,
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

    pub fn r#where<T: 'q + Encode<'q, E::T> + Type<E::T>>(&mut self, column: &str, operator: &str, val: T) -> &mut Self {
        if self.statement.query.where_queries.len() != 0 {
            return self.and_where(column, operator, val);
        }

        // TODO: find better name...
        self.statement.query.where_queries.push(WhereQuery {
            column: Some(column.to_string()),
            operator: Some(operator.to_string()),
            position: None,
            group: None
        });

        self.statement.arguments.add(val).unwrap();
        
        return self;
    }

    pub fn and_where<T: 'q + Encode<'q, E::T> + Type<E::T>>(&mut self, column: &str, operator: &str, val: T) -> &mut Self {
        if self.statement.query.where_queries.len() == 0 {
            return self.r#where(column, operator, val);
        }

        self.statement.query.where_queries.push(WhereQuery {
            column: Some(column.to_string()),
            operator: Some(operator.to_string()),
            position: Some(query::QueryPosition::AND),
            group: None
        });

        self.statement.arguments.add(val).unwrap();
        
        return self;
    }

    pub fn or_where<T: 'q + Encode<'q, E::T> + Type<E::T>>(&mut self, column: &str, operator: &str, val: T) -> &mut Self {
        if self.statement.query.where_queries.len() == 0 {
            return self.r#where(column, operator, val);
        }

        self.statement.query.where_queries.push(WhereQuery {
            column: Some(column.to_string()),
            operator: Some(operator.to_string()),
            position: Some(query::QueryPosition::OR),
            group: None
        });

        self.statement.arguments.add(val).unwrap();

        return self;
    }

    pub fn where_group(&mut self, callback: fn(group: WhereQueryGroup<'q, E::T>) -> WhereQueryGroup<'q, E::T>) -> &mut Self {        
        return self;
    }

    pub fn and_where_group(&mut self, callback: fn(group: WhereQueryGroup<'q, E::T>) -> WhereQueryGroup<'q, E::T>) -> &mut Self {        
        return self;
    }

    pub fn or_where_group(&mut self, callback: fn(group: WhereQueryGroup<'q, E::T>) -> WhereQueryGroup<'q, E::T>) -> &mut Self {        
        return self;
    }

    pub fn order_by(&mut self, column: &str, order: Order) -> &mut Self {
        self.statement.query.order_by.push(OrderQuery {
            column: column.to_string(),
            order: order
        });

        return self;
    }

    pub fn join(&mut self, table: &str, column: &str, column_table: &str) -> &mut Self {
        self.statement.query.join.push(JoinQuery {
            table: table.to_string(),
            column: column.to_string(),
            operator: "=".to_string(),
            column_table: column_table.to_string(),
            join_type: JoinType::LeftJoin 
        });

        return self;
    }

    pub fn limit(&mut self, limit: u64) -> &mut Self {
        self.statement.query.limit = Some(limit);

        return self;
    }



    pub fn bind<T: 'q + Encode<'q, E::T> + Type<E::T>>(&'q mut self, value: T) -> &'q mut Self {
        self.statement.arguments.add(value).unwrap();

        return self;
    }




    pub async fn query_all<O, T: 'q + Encode<'q, E::T> + Type<E::T>>(&'q mut self, sql: &str, args: Vec<T>) -> Result<Vec<O>>
    where
        O: for<'r> FromRow<'r, <E::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        return Ok(self.db.query_all::<O, T>(sql, args).await.unwrap());
    }

    pub async fn query_one<O, T: 'q + Encode<'q, E::T> + Type<E::T>>(&'q mut self, sql: &str, args: Vec<T>) -> Result<O>
    where
        O: for<'r> FromRow<'r, <E::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        return Ok(self.db.query_one::<O, T>(sql, args).await.unwrap())
    }

    pub async fn all<O>(&'q mut self) -> Result<Vec<O>>
    where
        O: for<'r> FromRow<'r, <E::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        return Ok(self.db.all::<O>(&self.statement).await.unwrap())
    }

    pub async fn paginate<O>(&'q mut self, limit: u64, page: u64) -> Result<Pagination<O>>
    where
        O: for<'r> FromRow<'r, <E::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        self.statement.query.limit = Some(limit);
        self.statement.query.page = Some(page); // TODO: calc offset using offset

        return Ok(self.db.paginate::<O>(&self.statement).await.unwrap());
    }

    pub fn to_sql(&'q mut self) -> Result<String> {
        return Ok(self.db.to_sql(&self.statement).unwrap())
    }



    pub fn insert_as<O>(&'q mut self, columns: Vec<&str>) -> InsertAs<'q, E, O>
    where
        O: for<'r> FromRow<'r, <E::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        return InsertAs::new(self.db, &mut self.statement);
    }
}

pub struct InsertAs<'q, E: Executor, O> {
    db: &'q E,
    statement: &'q mut Statement<'q, E::T>,
    _marker: PhantomData<E>,
    _type: PhantomData<O>
}

impl <'q, E, O>InsertAs<'q, E, O>
where
    E: Executor,
    O: for<'r> FromRow<'r, <E::T as sqlx::Database>::Row> + Send + Unpin + Sized
{
    pub(crate) fn new(db: &'q E, statement: &'q mut Statement<'q, E::T>) -> Self {
        return Self {
            db: db,
            statement: statement,
            _marker: PhantomData,
            _type: PhantomData
        }
    }

    pub fn bind<T: 'q + Encode<'q, E::T> + Type<E::T>>(&'q mut self, value: T) -> &'q mut Self {
        self.statement.arguments.add(value).unwrap();

        return self;
    }

    pub async fn execute(&'q mut self) -> Result<O> {
        return Ok(self.db.insert_as::<O>(self.statement).await.unwrap());
    }
}