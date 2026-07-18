use sqlx::SqlitePool;

use crate::Executor;

pub struct SQLite {
    pool: SqlitePool
}

impl SQLite {
    pub async fn new(url: impl Into<String>) -> Self {
        todo!()
    }
}

impl Executor for SQLite {
    type DB = sqlx::Sqlite;

    fn to_sql<'q>(&self, statement: &crate::Statement<Self::DB>) -> String {
        todo!()
    }
    
    async fn execute_as<'q, O>(&self, sql: String) -> crate::Result<Vec<O>>
    where
        O: crate::Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin {
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
    
    async fn query_all<'q, O>(&self, sql: &str) -> crate::Result<Vec<O>>
    where
        O: crate::Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin {
        todo!()
    }
    
    async fn query_one<'q, O>(&self, sql: &str) -> crate::Result<O>
    where
        O: crate::Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin {
        todo!()
    }

    async fn all<O>(&self, statement: &crate::Statement<Self::DB>) -> crate::Result<Vec<O>>
    where
        O: crate::Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin {
        todo!()
    }

    async fn first<O>(&self, statement: &crate::Statement<Self::DB>) -> crate::Result<O>
    where
        O: crate::Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin {
        todo!()
    }

    async fn get<O>(&self, statement: &crate::Statement<Self::DB>) -> crate::Result<Vec<O>>
    where
        O: crate::Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin {
        todo!()
    }

    async fn paginate<O>(&self, statement: &crate::Statement<Self::DB>) -> crate::Result<crate::Pagination<O>>
    where
        O: crate::Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin {
        todo!()
    }
}