use sqlx::{
    Arguments, Database as SqlxDatabase, PgPool, Postgres as Database,
};

use crate::{Entity, Executor, postgres::compile::Builder};

mod compile;

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
    type DB = Database;

    async fn first<'e, O: Entity>(&self, statement: &crate::Statement<Self::DB>) -> crate::Result<O>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as SqlxDatabase>::Row> + Send + Unpin {
        todo!()
    }
    
    async fn get<'e, O>(&self, statement: &crate::Statement<Self::DB>) -> crate::Result<Vec<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as SqlxDatabase>::Row> + Send + Unpin, 
    {
        let (sql, arguments) = Builder::new(statement).query();

        let results = sqlx::query_as_with::<Self::DB, O, _>(&sql, arguments)
            .fetch_all(&self.pool)
            .await?;
        return Ok(results);

        // todo!()
    }
    
}