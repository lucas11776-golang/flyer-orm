use crate::{
    Entity, Executor, Result, SqlitePool,
    database::{Builder, sqlite::builder::QueryBuilder},
    query::Statement,
    types::QueryResult,
    utils::to_args,
};

mod builder;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SQLiteQueryResult {
    pub(crate) affected: u64,
    pub(crate) id: u64,
}

impl SQLiteQueryResult {
    pub fn new(affected: u64, id: u64) -> Self {
        Self { affected, id }
    }
}

impl QueryResult for SQLiteQueryResult {
    fn rows_affected(&self) -> u64 {
        self.affected
    }

    fn last_inserted(&self) -> u64 {
        self.id
    }
}

pub struct SQLite {
    pool: SqlitePool,
}

impl SQLite {
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}

impl Executor for SQLite {
    type DB = sqlx::Sqlite;

    async fn new(url: &str) -> Result<Self>
    where
        Self: Sized
    {
        SqlitePool::connect(url)
            .await
            .map(|pool| Self { pool })
            .map_err(Into::into)
    }

    fn from(pool: sqlx::Pool<Self::DB>) -> Self {
        Self { pool }
    }

    fn builder<'a>(&self, dry_run: bool) -> impl Builder<'a, Self::DB> {
        QueryBuilder::new(dry_run)
    }

    fn to_sql<'q>(&self, statement: &Statement<Self::DB>) -> String {
        QueryBuilder::new(true).to_sql(statement)
    }

    fn pool(&self) -> &sqlx::Pool<Self::DB> {
        &self.pool
    }

    async fn execute<'c>(
        &'c self,
        sql: String,
        arguments: <Self::DB as sqlx::Database>::Arguments<'c>,
    ) -> Result<impl QueryResult> {
        let result = sqlx::query_with::<Self::DB, _>(&sql, arguments)
            .execute(&self.pool)
            .await?;

        Ok(SQLiteQueryResult::new(
            result.rows_affected(),
            result.last_insert_rowid() as u64,
        ))
    }

    async fn insert<'q>(&'q self, statement: &'q Statement<Self::DB>) -> Result<impl QueryResult> {
        let (sql, arguments) = QueryBuilder::new(false).insert(statement);

        self
            .execute(sql, to_args(arguments))
            .await
    }

    async fn insert_as<'q, O>(&'q self, statement: &'q Statement<Self::DB>) -> Result<O>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin,
    {
        let (sql, arguments) = QueryBuilder::new(false).insert(statement);

        let res = sqlx::query_with::<Self::DB, _>(&sql, to_args(arguments))
            .execute(&self.pool)
            .await?;

        let last_id = res.last_insert_rowid();

        let fetch_sql = format!("SELECT * FROM {} WHERE rowid = ?", statement.table);

        sqlx::query_as::<Self::DB, O>(&fetch_sql)
            .bind(last_id)
            .fetch_one(&self.pool)
            .await
            .map_err(Into::into)
    }

    async fn update<'q>(&'q self, statement: &'q Statement<Self::DB>) -> Result<impl QueryResult> {
        let (sql, arguments) = QueryBuilder::new(false).update(statement);

        self
            .execute(sql, to_args(arguments))
            .await
    }

    async fn delete<'q>(&'q self, statement: &'q Statement<Self::DB>) -> Result<impl QueryResult> {
        let (sql, arguments) = QueryBuilder::new(false).delete(statement);

        self
            .execute(sql, to_args(arguments))
            .await
    }

    async fn count<'q>(&'q self, statement: &'q Statement<Self::DB>) -> Result<i64> {
        let (sql, arguments) = QueryBuilder::new(false)
            .select(&["COUNT(*) AS total".into()])
            .from(&statement.table)
            .joins(&statement.join)
            .conditions(&statement.conditions, true)
            .group_by(&statement.group_by)
            .having(&statement.having)
            .compile();

        sqlx::query_scalar_with::<Self::DB, i64, _>(&sql, to_args(arguments))
            .fetch_one(&self.pool)
            .await
            .map(|total| total)
            .map_err(Into::into)
    }
}