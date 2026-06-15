use anyhow::Result;
use sqlx::{Arguments, Pool, Sqlite};

use crate::{
    Executor,
    databases::sqlite::{builder::Builder, query::SQLiteQueryResult},
    query::{Pagination, QueryBuilder, QueryResult, Statement},
    types::Where
};

pub mod query;
mod builder;


#[derive(Debug, sqlx::FromRow)]
pub(crate) struct SQLiteTotal {
    pub total: u64
}

#[derive(Debug)]
pub struct SQLite {
    db: Pool<Sqlite>,
}

impl SQLite {
    async fn fetch_one<'q, O>(&'q self, sql: String, arguments: <<Self as Executor>::T as sqlx::Database>::Arguments<'q>) -> Result<O>
    where
        O: for<'r> sqlx::FromRow<'r, <<Self as Executor>::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        return sqlx::query_as_with::<<Self as Executor>::T, O, _>(sql.as_str(), arguments)
            .fetch_one(&self.db)
            .await
            .map_err(|e| e.into());
    }

    async fn fetch_all<'q, O>(&'q self, sql: String, arguments: <<Self as Executor>::T as sqlx::Database>::Arguments<'q>) -> Result<Vec<O>>
    where
        O: for<'r> sqlx::FromRow<'r, <<Self as Executor>::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        return sqlx::query_as_with::<<Self as Executor>::T, O, _>(sql.as_str(), arguments)
            .fetch_all(&self.db)
            .await
            .map_err(|e| e.into());
    }

    async fn execute_query<'q>(&'q self, sql: String, arguments: <<Self as Executor>::T as sqlx::Database>::Arguments<'q>) -> Result<SQLiteQueryResult> {
        return sqlx::query_with::<<Self as Executor>::T, _>(sql.as_str(), arguments)
            .execute(&self.db)
            .await
            .map_err(|e| e.into())
            .map(|result| {
                return SQLiteQueryResult {
                    affected: result.rows_affected(),
                    id: result.last_insert_rowid() as u64,
                };
            });
    }
}

impl Executor for SQLite {
    type T = sqlx::Sqlite;

    async fn new(url: &str) -> Self where Self: Sized {
        return Self {
            db: sqlx::SqlitePool::connect(url).await.unwrap(),
        };
    }
    
    fn db<'q>(&'q self) -> &'q Pool<Self::T> {
        return &self.db;
    }
    
    fn to_sql<'q>(&self, statement: &'q Statement<'q, Self::T>) -> String {
        return Builder::new(&statement.query).query();
    }

    async fn execute<'q>(&self, sql: String, args: <Self::T as sqlx::Database>::Arguments<'q>) -> Result<impl QueryResult> {
        return self.execute_query(String::from(sql), args)
            .await;
    }
    
    async fn insert<'q>(&self, statement: &'q Statement<'q, Self::T>) -> Result<()> {
        return self.execute_query(Builder::new(&statement.query).insert(), statement.arguments.clone())
            .await
            .map(|_| ());
    }
    
    async fn insert_as<'q, O>(&self, statement: &'q Statement<'q, Self::T>) -> Result<O>
    where
        O: for<'r> sqlx::FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        let result = self.execute_query(Builder::new(&statement.query).insert(), statement.arguments.clone()).await.unwrap();

        let mut statement = Statement::<Self::T>::new(&statement.query.table);

        statement.query.where_queries.push(Where {
            column: Some("rowid".to_string()),
            operator: Some("=".to_string()),
            condition: None,
            group: None
        });

        statement.arguments.add(result.last_inserted() as i64).unwrap();

        return Ok(self.first(&statement).await.unwrap());
    }
    
    async fn update<'q>(&self, statement: &'q Statement<'q, Self::T>) -> Result<()> {
        return self.execute_query(Builder::new(&statement.query).update(), statement.arguments.clone())
            .await
            .map(|_| ());
    }
    
    async fn count<'q>(&self, statement: &'q Statement<'q, Self::T>) -> Result<u64> {
        let query = {
            let mut query = statement.query.clone();
            query.select = vec!["COUNT(*) as total".to_string()];
            query
        };

        return self.fetch_one::<SQLiteTotal>(Builder::new(&query).query(), statement.arguments.clone())
            .await
            .map(|t| t.total as u64);
    }
    
    async fn delete<'q>(&self, statement: &'q Statement<'q, Self::T>) -> Result<()> {
        return self.execute_query(Builder::new(&statement.query).delete(), statement.arguments.clone())
            .await
            .map(|_| ());
    }
    
    async fn query_all<'q, O, T: 'q + sqlx::Encode<'q, Self::T> + sqlx::Type<Self::T>>(&self, sql: &str, args: Vec<T>) -> Result<Vec<O>>
    where
        O: for<'r> sqlx::FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        let mut arguments: <Self::T as sqlx::Database>::Arguments<'q> = Default::default();

        for arg in args { arguments.add(arg).unwrap(); }

        return self.fetch_all(String::from(sql), arguments).await;
    }
    
    async fn query_one<'q, O, T: 'q + sqlx::Encode<'q, Self::T> + sqlx::Type<Self::T>>(&self, sql: &str, args: Vec<T>) -> Result<O>
    where
        O: for<'r> sqlx::FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        let mut arguments: <Self::T as sqlx::Database>::Arguments<'q> = Default::default();

        for arg in args { arguments.add(arg).unwrap(); }

        return self.fetch_one(String::from(sql), arguments).await;
    }
    
    async fn first<'q, O>(&self, statement: &'q Statement<'q, Self::T>) -> Result<O>
    where
        O: for<'r> sqlx::FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        return self.fetch_one(self.to_sql(statement), statement.arguments.clone()).await;
    }
    
    async fn all<'q, O>(&self, statement: &'q Statement<'q, Self::T>) -> Result<Vec<O>>
    where
        O: for<'r> sqlx::FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        return self.fetch_all::<O>(self.to_sql(statement), statement.arguments.clone()).await;
    }

    async fn paginate<'q, O>(&self, statement: &'q Statement<'q, Self::T>) -> Result<Pagination<O>>
    where
        O: for<'r> sqlx::FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        let mut query = statement.query.clone();

        query.select = vec!["COUNT(*) as total".to_string()];
        query.limit = None;
        query.offset = None;

        return Ok(
            Pagination {
                page: statement.query.offset.unwrap() as u64,
                per_page: statement.query.limit.unwrap() as u64,
                total: self.fetch_one::<SQLiteTotal>(Builder::new(&query).query(), statement.arguments.clone()).await.unwrap().total as u64,
                items: self.fetch_all::<O>(self.to_sql(statement), statement.arguments.clone()).await.unwrap(),
            }
        );
    }
}