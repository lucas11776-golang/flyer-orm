use std::{collections::HashMap, marker::PhantomData, str, sync::LazyLock};

use anyhow::{Ok, Result};
use sqlx::{Arguments, Encode, FromRow, types::Type};

use crate::{
    executor::Executor,
    query::{
        Order,
        Pagination,
        QueryResult,
        Statement,
        Transaction,
        insert::Insert,
        insert_as::InsertAs,
        raw_query::RawQuery,
        update::Update},
    types::{Condition, Join, JoinType, Where}
};

pub mod databases;
pub mod query;
pub mod executor;
pub mod types;

pub(crate) static mut CONNECTIONS: LazyLock<HashMap<&str, String>> = LazyLock::new(|| HashMap::new());

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
    pub async fn db<E: Executor>(connection: &str) -> Database::<E> {
        return unsafe { Database::new(CONNECTIONS.get(connection).unwrap()).await };
    }

    pub async fn db_with_url<E: Executor>(url: &str) -> Database::<E> {
        return Database::new(url).await;
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

    pub async fn execute<'q>(&'q self, sql: &str) -> Result<impl QueryResult>  {
        return self.executor
            .execute(String::from(sql), Default::default())
            .await;
    }

    pub fn raw_query<'q>(&'q self, sql: &str) -> RawQuery<'q, E> {
        return RawQuery::new(&self.executor, String::from(sql));
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
            statement: Statement::<E::T>::new(table),
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

    fn where_push<T: 'q + Encode<'q, E::T> + Type<E::T>>(&mut self, condition: Option<Condition>, column: &str, operator: &str, val: T) -> &mut Self {
        self.statement.query.where_queries.push(Where {
            condition: condition,
            column: Some(String::from(column)),
            operator: Some(String::from(operator)),
            group: None
        });
        self.statement.arguments.add(val).unwrap();
        return self;
    }

    pub fn r#where<T: 'q + Encode<'q, E::T> + Type<E::T>>(&mut self, column: &str, operator: &str, val: T) -> &mut Self {
        if self.statement.query.where_queries.len() != 0 {
            return self.and_where(column, operator, val);
        }
        return self.where_push(None, column, operator, val);
    }

    pub fn and_where<T: 'q + Encode<'q, E::T> + Type<E::T>>(&mut self, column: &str, operator: &str, val: T) -> &mut Self {
        if self.statement.query.where_queries.len() == 0 {
            return self.r#where(column, operator, val);
        }
        return self.where_push(Some(Condition::AND), column, operator, val);
    }

    pub fn or_where<T: 'q + Encode<'q, E::T> + Type<E::T>>(&mut self, column: &str, operator: &str, val: T) -> &mut Self {
        if self.statement.query.where_queries.len() == 0 {
            return self.r#where(column, operator, val);
        }
        return self.where_push(Some(Condition::OR), column, operator, val);
    }

    // pub fn where_group(&mut self, callback: fn(group: WhereQueryGroup<'q, E::T>) -> WhereQueryGroup<'q, E::T>) -> &mut Self {        
    //     return self;
    // }

    // pub fn and_where_group(&mut self, callback: fn(group: WhereQueryGroup<'q, E::T>) -> WhereQueryGroup<'q, E::T>) -> &mut Self {        
    //     return self;
    // }

    // pub fn or_where_group(&mut self, callback: fn(group: WhereQueryGroup<'q, E::T>) -> WhereQueryGroup<'q, E::T>) -> &mut Self {        
    //     return self;
    // }

    pub fn order_by(&mut self, column: &str, order: Order) -> &mut Self {
        if self.statement.query.order_by.is_none() {
            self.statement.query.order_by = Some(vec![types::Order {
                column: column.to_string(),
                order: order
            }]);

            return self;
        }

        self.statement.query.order_by.as_mut().unwrap().push(types::Order {
            column: column.to_string(),
            order: order
        });

        return self;
    }

    fn join_push(&mut self, join_type: JoinType, table: &str, column: &str, operator: &str, column_table: &str) -> &mut Self {
        self.statement.query.join.push(Join {
            table: String::from(table),
            column: String::from(column),
            operator: String::from(operator),
            column_table: String::from(column_table),
            join_type: join_type
        });
        return self;
    }

    pub fn join(&mut self, table: &str, column: &str, operator: &str, column_table: &str) -> &mut Self {
        return self.join_push(JoinType::LeftJoin , table, column, operator, column_table);
    }

    pub fn join_inner(&mut self, table: &str, column: &str, operator: &str, column_table: &str) -> &mut Self {
        return self.join_push(JoinType::InnerJoin , table, column, operator, column_table);
    }

    pub fn join_right(&mut self, table: &str, column: &str, operator: &str, column_table: &str) -> &mut Self {
        return self.join_push(JoinType::RightJoin , table, column, operator, column_table);
    }

    pub fn join_full_outer(&mut self, table: &str, column: &str, operator: &str, column_table: &str) -> &mut Self {
        return self.join_push(JoinType::FullOuterJoin , table, column, operator, column_table);
    }

    pub fn join_cross(&mut self, table: &str, column: &str, operator: &str, column_table: &str) -> &mut Self {
        return self.join_push(JoinType::CrossJoin , table, column, operator, column_table);
    }

    pub fn limit(&mut self, limit: i64) -> &mut Self {
        self.statement.query.limit = Some(limit);

        return self;
    }

    pub fn bind<T: 'q + Encode<'q, E::T> + Type<E::T>>(&'q mut self, value: T) -> &'q mut Self {
        self.statement.arguments.add(value).unwrap();

        return self;
    }

    pub async fn query<O, T: 'q + Encode<'q, E::T> + Type<E::T>>(&'q mut self, sql: &str, args: Vec<T>) -> Result<Vec<O>>
    where
        O: for<'r> FromRow<'r, <E::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        return Ok(self.db.query_all::<O, T>(sql, args).await.unwrap());
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

    pub fn insert_as<O>(&'q mut self, columns: Vec<&str>) -> InsertAs<'q, E, O>
    where
        O: for<'r> FromRow<'r, <E::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        self.statement.query.select = columns.iter().map(|c| c.to_string()).collect();

        return InsertAs::new(self.db, &mut self.statement);
    }

    pub fn insert(&'q mut self, columns: Vec<&str>) -> Insert<'q, E> {
        self.statement.query.select = columns.iter().map(|c| c.to_string()).collect();

        return Insert::new(&self.db, &mut self.statement);
    }

    pub fn update(&'q mut self, columns: Vec<&str>) -> Update<'q, E> {
        self.statement.query.select = columns.iter().map(|c| c.to_string()).collect();

        return Update::new(&self.db, &mut self.statement);
    }

    pub async fn delete(&'q mut self) -> Result<()>
    {
        return self.db.delete(&self.statement).await;
    }

    pub async fn first<O>(&'q mut self) -> Result<O>
    where
        O: for<'r> FromRow<'r, <E::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        return self.db.first::<O>(&self.statement).await;
    }

    pub async fn all<O>(&'q mut self) -> Result<Vec<O>>
    where
        O: for<'r> FromRow<'r, <E::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        return self.db.all::<O>(&self.statement).await;
    }

    pub async fn count(&'q mut self) -> Result<u64> {
        return self.db.count(&self.statement).await;
    }

    pub async fn exists(&'q mut self) -> Result<bool> {
        return self.db.count(&self.statement).await.map(|t| t > 0);
    }

    pub async fn paginate<O>(&'q mut self, limit: i64, page: i64) -> Result<Pagination<O>>
    where
        O: for<'r> FromRow<'r, <E::T as sqlx::Database>::Row> + Send + Unpin + Sized,
        i64: Encode<'q, E::T> + Type<E::T>
    {
        self.statement.query.limit = Some(limit);
        self.statement.query.offset = Some(page); // TODO: calc offset using offset

        self.statement.arguments.add(limit).unwrap();
        self.statement.arguments.add(if page > 1 {
            (page - 1) * limit 
        } else {
            0
        }).unwrap();

        return self.db.paginate::<O>(&self.statement).await;
    }

    pub fn to_sql(&'q mut self) -> String {
        return self.db.to_sql(&self.statement);
    }
}