use crate::{
    SqlitePool,
    Entity,
    Executor,
    Pagination,
    QueryResult,
    Result,
    database::sqlite::builder::QueryBuilder,
    query::Statement,
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

    async fn new(url: impl Into<String>) -> Self {
        Self {
            pool: SqlitePool::connect(&url.into())
                .await
                .unwrap(),
        }
    }

    fn from(pool: sqlx::Pool<Self::DB>) -> Self {
        Self { pool }
    }

    fn to_sql<'q>(&self, statement: &Statement<Self::DB>) -> String {
        QueryBuilder::new(true).to_sql(statement)
    }

    fn db(&self) -> &sqlx::Pool<Self::DB> {
        &self.pool
    }

    async fn execute<'c>(
        &self,
        sql: String,
        arguments: <Self::DB as sqlx::Database>::Arguments<'c>,
    ) -> Result<impl crate::QueryResult> {
        let result = sqlx::query_with::<Self::DB, _>(&sql, arguments)
            .execute(&self.pool)
            .await?;

        Ok(SQLiteQueryResult::new(
            result.rows_affected(),
            result.last_insert_rowid() as u64,
        ))
    }

    async fn fetch_one<'c, O>(
        &self,
        sql: String,
        arguments: <Self::DB as sqlx::Database>::Arguments<'c>,
    ) -> Result<O>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin,
    {
        sqlx::query_as_with::<Self::DB, O, _>(&sql, arguments)
            .fetch_one(&self.pool)
            .await
            .map_err(Into::into)
    }

    async fn fetch_all<'c, O>(
        &self,
        sql: String,
        arguments: <Self::DB as sqlx::Database>::Arguments<'c>,
    ) -> Result<Vec<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin,
    {
        sqlx::query_as_with::<Self::DB, O, _>(&sql, arguments)
            .fetch_all(&self.pool)
            .await
            .map_err(Into::into)
    }

    async fn insert<'q>(&self, statement: &Statement<Self::DB>) -> Result<impl QueryResult> {
        let (sql, arguments) = QueryBuilder::new(false).insert(statement);

        self
            .execute(sql, arguments)
            .await
    }

    async fn insert_as<'q, O>(&self, statement: &Statement<Self::DB>) -> Result<O>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin,
    {
        let (sql, arguments) = QueryBuilder::new(false).insert(statement);

        // 1. Execute insert statement and retrieve generated rowid
        let res = sqlx::query_with::<Self::DB, _>(&sql, arguments)
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

    async fn update<'q>(&self, statement: &Statement<Self::DB>) -> Result<impl QueryResult> {
        let (sql, arguments) = QueryBuilder::new(false).update(statement);

        self
            .execute(sql, arguments)
            .await
    }

    async fn delete<'q>(&self, statement: &Statement<Self::DB>) -> Result<impl QueryResult> {
        let (sql, arguments) = QueryBuilder::new(false).delete(statement);

        self
            .execute(sql, arguments)
            .await
    }

    async fn count<'q>(&self, statement: &Statement<Self::DB>) -> Result<i64> {
        let (sql, arguments) = QueryBuilder::new(false)
            .select(&["COUNT(*) AS total".into()])
            .from(&statement.table)
            .joins(&statement.join)
            .conditions(&statement.conditions, true)
            .group_by(&statement.group_by)
            .having(&statement.having)
            .compile();

        sqlx::query_scalar_with::<Self::DB, i64, _>(&sql, arguments)
            .fetch_one(&self.pool)
            .await
            .map(|total| total)
            .map_err(Into::into)
    }

    async fn first<O>(&self, statement: &Statement<Self::DB>) -> Result<O>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin,
    {
        let (sql, arguments) = QueryBuilder::new(false).query(statement);

        self.fetch_one(sql, arguments).await
    }

    async fn all<O>(&self, statement: &Statement<Self::DB>) -> Result<Vec<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin,
    {
        let (sql, arguments) = QueryBuilder::new(false).query(statement);

        self.fetch_all(sql, arguments).await
    }

    async fn paginate<O>(&self, statement: &Statement<Self::DB>) -> Result<Pagination<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin,
    {
        let (items, total) = tokio::try_join!(
            self.all::<O>(statement),
            self.count(statement)
        )?;

        Ok(Pagination {
            total: total,
            page: statement.page.unwrap(),
            per_page: statement.limit.as_ref().unwrap().value.parse().unwrap(),
            items: items,
        })
    }
}