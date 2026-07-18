use sqlx::{
    Database as SqlxDatabase,
    PgPool,
};

use crate::{Entity, Executor, postgres::builder::QueryBuilder};

mod builder;

pub struct Postgres {
    pool: PgPool
}

impl Postgres {
    pub async fn new(url: impl Into<String>) -> Self {
        return Self {
            pool: PgPool::connect(&url.into())
                .await
                .unwrap()
        }
    }
}

impl Executor for Postgres {
    type DB = sqlx::Postgres;
    
    fn to_sql<'q>(&self, statement: &crate::Statement<Self::DB>) -> String {
        let (sql, _) = QueryBuilder::new(true).query(statement);

        return sql; 
    }

    async fn all<O>(&self, statement: &crate::Statement<Self::DB>) -> crate::Result<Vec<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as SqlxDatabase>::Row> + Send + Unpin
    {
        let (sql, arguments) = QueryBuilder::new(false).query(statement);


        println!("\r\n\r\n\r\n\r\n ToSQL: {}", sql);

        return  sqlx::query_as_with::<Self::DB, O, _>(&sql, arguments)
            .fetch_all(&self.pool)
            .await
            .map_err(|err| err.into());
    }

    async fn first<O>(&self, statement: &crate::Statement<Self::DB>) -> crate::Result<O>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as SqlxDatabase>::Row> + Send + Unpin
    {
        let (sql, arguments) = QueryBuilder::new(false).query(statement);

        return  sqlx::query_as_with::<Self::DB, O, _>(&sql, arguments)
            .fetch_one(&self.pool)
            .await
            .map_err(|err| err.into());
    }
    
    async fn get<O>(&self, statement: &crate::Statement<Self::DB>) -> crate::Result<Vec<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as SqlxDatabase>::Row> + Send + Unpin, 
    {
        return self.all(statement).await;
    }
    
    async fn paginate<O>(&self, statement: &crate::Statement<Self::DB>) -> crate::Result<crate::Pagination<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as SqlxDatabase>::Row> + Send + Unpin
    {
        // let total = Builder::new(false)
        //     .

        todo!()
    }
}