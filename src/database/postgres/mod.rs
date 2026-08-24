use crate::{
    Entity, Executor, PgPool, QueryResult, Result, database::{Builder, postgres::builder::QueryBuilder}, query::Statement, utils::to_args,
};

mod builder;
pub mod decoders;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PostgresQueryResult {
    pub(crate) affected: u64,
    pub(crate) id: u64,
}

impl PostgresQueryResult {
    pub fn new(affected: u64, id: u64) -> Self {
        Self { affected, id }
    }
}

impl QueryResult for PostgresQueryResult {
    fn rows_affected(&self) -> u64 {
        self.affected
    }

    fn last_inserted(&self) -> u64 {
        self.id
    }
}

pub struct Postgres {
    pool: PgPool,
}

impl Executor for Postgres {
    type DB = sqlx::Postgres;

    async fn new(url: &str) -> Result<Self>
    where
        Self: Sized
    {
        PgPool::connect(url)
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
        self
            .builder(true)
            .to_sql(statement)
    }

    fn pool(&self) -> &sqlx::Pool<Self::DB> {
        &self.pool
    }

    async fn execute<'q>(
        &'q self,
        sql: String,
        arguments: <Self::DB as sqlx::Database>::Arguments<'q>,
    ) -> Result<impl crate::QueryResult> {
        sqlx::query_with::<Self::DB, _>(&sql, arguments)
            .execute(&self.pool)
            .await
            .map(|r| PostgresQueryResult::new(r.rows_affected(), 0))
            .map_err(Into::into)
    }

    async fn insert<'a>(&'a self, statement: &'a Statement<Self::DB>) -> Result<impl QueryResult> {
        let (sql, arguments) = QueryBuilder::new(false).insert(statement);

        self
            .execute(sql, to_args(arguments))
            .await
    }

    async fn insert_as<'a, O>(&self, statement: &'a Statement<Self::DB>) -> Result<O>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin,
    {
        let (mut sql, arguments) = QueryBuilder::new(false).insert(statement);
        sql.push_str(" RETURNING *");

        sqlx::query_as_with::<Self::DB, O, _>(&sql, to_args(arguments))
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

    async fn delete<'a>(&self, statement: &'a Statement<Self::DB>) -> Result<impl QueryResult> {
        let (sql, arguments) = QueryBuilder::new(false).delete(statement);

        self
            .execute(sql, to_args(arguments))
            .await
    }

    async fn count<'a>(&self, statement: &'a Statement<Self::DB>) -> Result<i64> {
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