use sqlx::SqlitePool;

use crate::{
    Entity,
    Executor,
    Pagination,
    QueryResult,
    database::sqlite::builder::QueryBuilder,
};

mod builder;

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

#[derive(crate::Entity)]
struct Total {
    pub total: i64,
}

pub struct SQLite {
    pool: sqlx::SqlitePool
}

impl SQLite {
}

impl Executor for SQLite {
    type DB = sqlx::Sqlite;

    async fn new(url: impl Into<String>) -> Self {
        Self {
            pool: SqlitePool::connect(&url.into())
                .await
                .unwrap()
        }
    }
    
    fn from(pool: sqlx::Pool<Self::DB>) -> Self {
        Self { pool: pool }
    }

    fn to_sql<'q>(&self, statement: &crate::Statement<Self::DB>) -> String {
        QueryBuilder::new(true).to_sql(statement)
    }
    
    fn db(&self) -> &sqlx::Pool<Self::DB> {
        &self.pool
    }
    
    async fn execute<'c>(&self, sql: String, arguments: <Self::DB as sqlx::Database>::Arguments<'c>) -> crate::Result<impl crate::QueryResult> {
        sqlx::query_with(&sql, arguments)
            .execute(&self.pool)
            .await
            .map_err(|err| err.into())
            .map(|result| SQLiteQueryResult::new(result.rows_affected(), result.last_insert_rowid() as u64))
    }

    async fn fetch_one<'c, O>(&self, sql: String, arguments: <Self::DB as sqlx::Database>::Arguments<'c>) -> crate::Result<O>
    where
        O: crate::Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin
    {
        sqlx::query_as_with::<Self::DB, O, _>(&sql, arguments)
            .fetch_one(&self.pool)
            .await
            .map_err(|err| err.into())
    }
    
    async fn fetch_all<'c, O>(&self, sql: String, arguments: <Self::DB as sqlx::Database>::Arguments<'c>) -> crate::Result<Vec<O>>
    where
        O: crate::Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin
    {
        sqlx::query_as_with::<Self::DB, O, _>(&sql, arguments)
            .fetch_all(&self.pool)
            .await
            .map_err(|err| err.into())
    }
    
    async fn insert<'q>(&self, statement: &crate::Statement<Self::DB>) -> crate::Result<impl QueryResult> {
        let (sql, arguments) = QueryBuilder::new(false).insert(statement);

        self
            .execute(sql, arguments)
            .await
    }
    
    async fn update<'q>(&self, _statement: &crate::Statement<Self::DB>) -> crate::Result<()> {
        todo!("Update is not yet implemented for SQLite")
    }
    
    async fn count<'q>(&self, statement: &crate::Statement<Self::DB>) -> crate::Result<u64> {
        let (sql, arguments) = QueryBuilder::new(false)
            .select(&vec!["COUNT(*) AS total".into()])
            .from(&statement.table)
            .joins(&statement.join)
            .conditions(&statement.conditions, true)
            .group_by(&statement.group_by)
            .having(&statement.having)
            .compile();

        sqlx::query_as_with::<Self::DB, Total, _>(&sql, arguments)
            .fetch_one(&self.pool)
            .await
            .map(|total| total.total as u64)
            .map_err(|err| err.into())
    }
    
    async fn delete<'q>(&self, _statement: &crate::Statement<Self::DB>) -> crate::Result<()> {
        todo!("Delete is not yet implemented for SQLite")
    }
    
    async fn insert_as<'q, O>(&self, _statement: &crate::Statement<Self::DB>) -> crate::Result<O>
    where
        O: crate::Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin {
        todo!("Insert as is not yet implemented for SQLite")
    }

    async fn first<O>(&self, statement: &crate::Statement<Self::DB>) -> crate::Result<O>
    where
        O: crate::Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin
    {
        let (sql, arguments) = QueryBuilder::new(false).query(statement);

        self
            .fetch_one(sql, arguments)
            .await
    }

    async fn all<O>(&self, statement: &crate::Statement<Self::DB>) -> crate::Result<Vec<O>>
    where
        O: crate::Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin {
        let (sql, arguments) = QueryBuilder::new(false).query(statement);

        self
            .fetch_all(sql, arguments)
            .await
    }

    async fn paginate<O>(&self, statement: &crate::Statement<Self::DB>) -> crate::Result<crate::Pagination<O>>
    where
        O: crate::Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin
    {
        let (items, total) = tokio::try_join!(
            self.all::<O>(statement),
            self.count(statement)
        )?;

        Ok(Pagination {
            total: total,
            page: 1,
            per_page: 10,
            items: items,
        })
    }
}
