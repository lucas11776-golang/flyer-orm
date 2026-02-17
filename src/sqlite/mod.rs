mod builder;


use anyhow::Result;
use sqlx::{Arguments, Pool, Sqlite};

use crate::{Executor, QueryBuilder, query::{Pagination, Statement, Total}, sqlite::builder::Builder};

#[derive(Debug)]
pub struct SQLite {
    db: Pool<Sqlite>,
    builder: Builder,
}

impl Executor for SQLite {
    type T = sqlx::Sqlite;

    async fn new(url: &str) -> Self where Self: Sized {
        return Self {
            db: sqlx::SqlitePool::connect(url).await.unwrap(),
            builder: Builder::default(),
        }
    }
    
    fn db<'q>(&'q self) -> &'q Pool<Self::T> {
        return &self.db;
    }
    
    fn to_sql<'q>(&self, statement: &'q Statement<'q, Self::T>) -> Result<String> {
        // return Ok(self.builder.build(&statement.query).unwrap());

        todo!()
    }
    
    async fn query_all<'q, O, T: 'q + sqlx::Encode<'q, Self::T> + sqlx::Type<Self::T>>(&self, sql: &str, args: Vec<T>) -> Result<Vec<O>>
    where
        O: for<'r> sqlx::FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        let mut arguments: <Self::T as sqlx::Database>::Arguments<'q> = Default::default();

        for arg in args {
            arguments.add(arg).unwrap();
        }

        return Ok(
            sqlx::query_as_with::<Self::T, O, _>(sql, arguments)
                .fetch_all(&self.db)
                .await
                .unwrap()
        )
    }
    
    async fn query_one<'q, O, T: 'q + sqlx::Encode<'q, Self::T> + sqlx::Type<Self::T>>(&self, sql: &str, args: Vec<T>) -> Result<O>
    where
        O: for<'r> sqlx::FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        let mut arguments: <Self::T as sqlx::Database>::Arguments<'q> = Default::default();

        for arg in args {
            arguments.add(arg).unwrap();
        }

        return Ok(
            sqlx::query_as_with::<Self::T, O, _>(sql, arguments)
                .fetch_one(&self.db)
                .await
                .unwrap()
        )
    }
    
    async fn first<'q, O>(&self, statement: &'q Statement<'q, Self::T>) -> Result<O>
    where
        O: for<'r> sqlx::FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        return Ok(
            sqlx::query_as_with::<Self::T, O, _>(&self.to_sql(statement).unwrap(), statement.arguments.clone())
                .fetch_one(&self.db)
                .await
                .unwrap()
        );
    }
    
    async fn all<'q, O>(&self, statement: &'q Statement<'q, Self::T>) -> Result<Vec<O>>
    where
        O: for<'r> sqlx::FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        return Ok(
            sqlx::query_as_with::<Self::T, O, _>(&self.to_sql(statement).unwrap(), statement.arguments.clone())
                .fetch_all(&self.db)
                .await
                .unwrap(),
        );
    }
    
    // TODO: refactor...
    async fn paginate<'q, O>(&self, statement: &'q Statement<'q, Self::T>) -> Result<Pagination<O>>
    where
        O: for<'r> sqlx::FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        let mut query = statement.query.clone();

        query.select = vec!["COUNT(*) as total".to_string()];
        query.limit = None;
        query.page = None;
        

        let total = sqlx::query_as_with::<Self::T, Total, _>(&self.to_sql(statement).unwrap(), statement.arguments.clone())
            .fetch_one(&self.db)
            .await
            .unwrap();

        let items = sqlx::query_as_with::<Self::T, O, _>(&self.to_sql(statement).unwrap(), statement.arguments.clone())
            .fetch_all(&self.db)
            .await
            .unwrap();

        return Ok(
            Pagination {
                total: total.total,
                page: statement.query.page.unwrap(),
                per_page: statement.query.limit.unwrap(),
                items: items,
            }
        );
    }
}
