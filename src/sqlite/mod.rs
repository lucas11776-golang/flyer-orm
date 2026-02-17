mod builder;


use anyhow::Result;
use sqlx::Pool;

use crate::{Executor, QueryBuilder, query::{Pagination, Statement}, sqlite::builder::Builder};

pub struct SQLite<'q> {
    statement: &'q Statement<'q, <SQLite<'q> as Executor<'q>>::T>
}

impl <'q>Executor<'q> for SQLite<'q> {
    type T = sqlx::Sqlite;

    fn new(statement: &'q Statement<'q, Self::T>) -> Self where Self: Sized {
        return Self {
            statement: statement
        }
    }

    async fn db(&self, url: &str) -> Result<Pool<Self::T>> {
        return Ok(sqlx::SqlitePool::connect(url).await.unwrap());
    }
    
    fn to_sql(&self) -> Result<String> {
        return  Ok(Builder::build(self.statement).unwrap());
    }
    
    async fn first<O>(&self) -> Result<O>
    where
        O: for<'r> sqlx::FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        return Ok(
            sqlx::query_as::<Self::T, O>(&Builder::build(self.statement).unwrap())
                .fetch_one(&self.db(&self.statement.url).await.unwrap())
                .await
                .unwrap()
        );
    }
    
    async fn get<O>(&self) -> Result<Vec<O>>
    where
        O: for<'r> sqlx::FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {
        return Ok(
            sqlx::query_as::<Self::T, O>(&Builder::build(self.statement).unwrap())
                .fetch_all(&self.db(&self.statement.url).await.unwrap())
                .await
                .unwrap(),
        );
    }
    
    // TODO: extract QueryStatement -> remove (arguments) allow clone...
    async fn paginate<O>(&mut self) -> Result<Pagination<O>>
    where
        O: for<'r> sqlx::FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin + Sized
    {

        todo!();

        // let temp = (self.statement.select.clone(), self.statement.limit.clone(), self.statement.page.clone());

        // self.statement.select = vec!["COUNT(*) as total".to_string()];
        // self.statement.limit = None;
        // self.statement.page = None;

        // let total = self.first::<Total>().await.unwrap();

        // self.statement.select = temp.0;
        // self.statement.limit = None;
        // self.statement.page = None;

        // return Ok(
        //     Pagination {
        //         total: total.total,
        //         page: self.statement.page.unwrap(),
        //         per_page: self.statement.limit.unwrap(),
        //         items: self.get().await.unwrap(),
        //     },
        // );
    }
    

    
}

