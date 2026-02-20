mod builder;

use anyhow::Result;
use sqlx::{Arguments, Pool, Sqlite};

use crate::{Executor, query::{Pagination, QueryBuilder, Statement, Total, WhereQuery}, sqlite::builder::Builder};

#[derive(Debug)]
pub struct SQLite {
    db: Pool<Sqlite>,
}

impl Executor for SQLite {
    type T = sqlx::Sqlite;

    async fn new(url: &str) -> Self where Self: Sized {
        return Self {
            db: sqlx::SqlitePool::connect(url).await.unwrap(),
        }
    }
    
    fn db<'q>(&'q self) -> &'q Pool<Self::T> {
        return &self.db;
    }
    
    fn to_sql<'q>(&self, statement: &'q Statement<'q, Self::T>) -> Result<String> {
        return Ok(Builder::new(&statement.query).query().unwrap());
    }

    async fn execute<'q>(&self, sql: &'q str) -> Result<()> {
        todo!();
    }
    
    async fn insert<'q>(&self, statement: &'q Statement<'q, Self::T>) -> Result<()> {
        sqlx::query_with::<Self::T, _>(&Builder::new(&statement.query).insert().unwrap(), statement.arguments.clone())
            .execute(&self.db)
            .await
            .unwrap();
        return Ok(());
    }
    
    async fn insert_as<'q, O>(&self, statement: &'q Statement<'q, Self::T>) -> Result<O>
    where
        O: for<'r> sqlx::FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {   
        let query_result = sqlx::query_with::<Self::T, _>(&Builder::new(&statement.query).insert().unwrap(), statement.arguments.clone())
            .execute(&self.db)
            .await
            .unwrap();
        
        let mut statement = Statement::<Self::T>::new(&statement.query.table);

        statement.query.where_queries.push(WhereQuery {
            column: Some("rowid".to_string()),
            operator: Some("=".to_string()),
            position: None,
            group: None
        });

        statement.arguments.add(query_result.last_insert_rowid()).unwrap();

        return Ok(self.first(&statement).await.unwrap());
    }
    
    async fn update<'q>(&self, statement: &'q Statement<'q, Self::T>) -> Result<()> {
        sqlx::query_with::<Self::T, _>(&Builder::new(&statement.query).update().unwrap(), statement.arguments.clone())
            .execute(&self.db)
            .await
            .unwrap();
        return Ok(());
    }
    
    async fn count<'q>(&self, statement: &'q Statement<'q, Self::T>) -> Result<u64> {
        return Ok(0);
    }
    
    async fn delete<'q>(&self, statement: &'q Statement<'q, Self::T>) -> Result<()> {
        sqlx::query_with::<Self::T, _>(&Builder::new(&statement.query).delete().unwrap(), statement.arguments.clone())
            .execute(&self.db)
            .await
            .unwrap();
        return Ok(());
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

        return Ok(
            Pagination {
                page: statement.query.page.unwrap(),
                per_page: statement.query.limit.unwrap(),
                total: sqlx::query_as_with::<Self::T, Total, _>(&self.to_sql(statement).unwrap(), statement.arguments.clone())
                    .fetch_one(&self.db)
                    .await
                    .unwrap()
                    .total,
                items: sqlx::query_as_with::<Self::T, O, _>(&self.to_sql(statement).unwrap(), statement.arguments.clone())
                    .fetch_all(&self.db)
                    .await
                    .unwrap(),
            }
        );
    }
}


