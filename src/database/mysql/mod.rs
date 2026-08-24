use crate::{
    Entity, Executor, MySqlPool, QueryResult, Result, database::{Builder, mysql::builder::QueryBuilder}, query::Statement, utils::to_args,
};

mod builder;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MySqlQueryResult {
    pub(crate) affected: u64,
    pub(crate) id: u64,
}

impl MySqlQueryResult {
    pub fn new(affected: u64, id: u64) -> Self {
        Self { affected, id }
    }
}

impl QueryResult for MySqlQueryResult {
    fn rows_affected(&self) -> u64 {
        self.affected
    }

    fn last_inserted(&self) -> u64 {
        self.id
    }
}

pub struct MySQL {
    pool: MySqlPool,
}

impl MySQL {
    pub fn pool(&self) -> &MySqlPool {
        &self.pool
    }
}

impl Executor for MySQL {
    type DB = sqlx::MySql;

    async fn new(url: &str) -> Result<Self>
    where
        Self: Sized
    {
        MySqlPool::connect(url)
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

    fn to_sql<'a>(&'a self, statement: &'a Statement<Self::DB>) -> String {
        QueryBuilder::new(true).to_sql(statement)
    }

    fn pool(&self) -> &sqlx::Pool<Self::DB> {
        &self.pool
    }

    async fn execute<'a>(
        &'a self,
        sql: String,
        arguments: <Self::DB as sqlx::Database>::Arguments<'a>,
    ) -> Result<impl crate::QueryResult> {
        let result = sqlx::query_with::<Self::DB, _>(&sql, arguments)
            .execute(&self.pool)
            .await?;

        Ok(MySqlQueryResult::new(
            result.rows_affected(),
            result.last_insert_id(),
        ))
    }
    
    async fn insert<'a>(&'a self, statement: &'a Statement<Self::DB>) -> Result<impl QueryResult> {
        let (sql, arguments) = QueryBuilder::new(false).insert(statement);

        self
            .execute(sql, to_args(arguments))
            .await
    }

    async fn insert_as<'a, O>(&'a self, statement: &'a Statement<Self::DB>) -> Result<O>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin,
    {
        let (sql, arguments) = QueryBuilder::new(false).insert(statement);

        let res = sqlx::query_with::<Self::DB, _>(&sql, to_args(arguments))
            .execute(&self.pool)
            .await?;

        let last_id = res.last_insert_id();

        let fetch_sql = format!("SELECT * FROM {} WHERE id = ?", statement.table);

        sqlx::query_as::<Self::DB, O>(&fetch_sql)
            .bind(last_id)
            .fetch_one(&self.pool)
            .await
            .map_err(Into::into)
    }

    async fn update<'a>(&'a self, statement: &'a Statement<Self::DB>) -> Result<impl QueryResult> {
        let (sql, arguments) = QueryBuilder::new(false).update(statement);

        self
            .execute(sql, to_args(arguments))
            .await
    }

    async fn delete<'a>(&'a self, statement: &'a Statement<Self::DB>) -> Result<impl QueryResult> {
        let (sql, arguments) = QueryBuilder::new(false).delete(statement);

        self
            .execute(sql, to_args(arguments))
            .await
    }

    async fn count<'a>(&'a self, statement: &'a Statement<Self::DB>) -> Result<i64> {
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