use sqlx::{
    Database as SqlxDatabase,
    PgPool,
    Postgres as Database,
    postgres::PgArguments
};

use crate::{Entity, Executor};

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
        let (sql, args) = ("SELECT * FROM users", PgArguments::default());
        let results = sqlx::query_as_with::<Self::DB, O, _>(&sql, args)
            .fetch_all(&self.pool)
            .await?;
        return Ok(results);

        // todo!()
    }
    
}