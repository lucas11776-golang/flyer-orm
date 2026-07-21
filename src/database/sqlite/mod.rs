use sqlx::SqlitePool;

use crate::{Executor, QueryResult};

pub struct SQLiteQueryResult {
    pub(crate) affected: u64,
    pub(crate) id: u64,
}

impl SQLiteQueryResult {
    pub fn new(affected: u64, id: u64) -> Self {
        return Self {
            affected: affected,
            id: id,
        };
    }
}

impl QueryResult for SQLiteQueryResult {
    fn rows_affected(&self) -> u64 {
        return self.affected;
    }

    fn last_inserted(&self) -> u64 {
        return self.id;
    }
}

pub struct SQLite {
    _pool: SqlitePool
}

impl SQLite {
    pub async fn new(url: impl Into<String>) -> Self {
        todo!()
    }
}

impl Executor for SQLite {
    type DB = sqlx::Sqlite;

    fn to_sql<'q>(&self, _statement: &crate::Statement<Self::DB>) -> String {
        todo!()
    }
    
    async fn execute<'c>(&self, sql: String, arguments: <Self::DB as sqlx::Database>::Arguments<'c>) -> crate::Result<impl crate::QueryResult> {
        return Ok(SQLiteQueryResult::new(0, 0));
    }

    async fn fetch_one<'c, O>(&self, sql: String, arguments: <Self::DB as sqlx::Database>::Arguments<'c>) -> crate::Result<O>
    where
        O: crate::Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin
    {
        todo!()
    }
    
    async fn fetch_all<'c, O>(&self, sql: String, arguments: <Self::DB as sqlx::Database>::Arguments<'c>) -> crate::Result<Vec<O>>
    where
        O: crate::Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin
    {
        todo!()
    }
    
    async fn insert<'q>(&self, statement: &crate::Statement<Self::DB>) -> crate::Result<()> {
        todo!()
    }
    
    async fn update<'q>(&self, statement: &crate::Statement<Self::DB>) -> crate::Result<()> {
        todo!()
    }
    
    async fn count<'q>(&self, statement: &crate::Statement<Self::DB>) -> crate::Result<u64> {
        todo!()
    }
    
    async fn delete<'q>(&self, statement: &crate::Statement<Self::DB>) -> crate::Result<()> {
        todo!()
    }
    
    async fn insert_as<'q, O>(&self, statement: &crate::Statement<Self::DB>) -> crate::Result<O>
    where
        O: crate::Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin {
        todo!()
    }

    async fn all<O>(&self, _statement: &crate::Statement<Self::DB>) -> crate::Result<Vec<O>>
    where
        O: crate::Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin {
        todo!()
    }

    async fn first<O>(&self, _statement: &crate::Statement<Self::DB>) -> crate::Result<O>
    where
        O: crate::Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin {
        todo!()
    }

    async fn get<O>(&self, _statement: &crate::Statement<Self::DB>) -> crate::Result<Vec<O>>
    where
        O: crate::Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin {
        todo!()
    }

    async fn paginate<O>(&self, _statement: &crate::Statement<Self::DB>) -> crate::Result<crate::Pagination<O>>
    where
        O: crate::Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin {
        todo!()
    }
}