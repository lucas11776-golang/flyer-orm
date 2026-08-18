use crate::{
    PgPool,
    Entity,
    Executor,
    Pagination,
    QueryResult,
    Result,
    database::postgres::builder::QueryBuilder,
    query::Statement,
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

impl Postgres {
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}

impl Executor for Postgres {
    type DB = sqlx::Postgres;

    async fn new(url: impl Into<String>) -> Self {
        Self {
            pool: PgPool::connect(&url.into())
                .await
                .unwrap(),
        }
    }

    fn from(pool: sqlx::Pool<Self::DB>) -> Self {
        Self { pool }
    }

    fn to_sql<'q>(&'q self, statement: &Statement<Self::DB>) -> String {
        QueryBuilder::new(true).to_sql(statement)
    }

    fn pool(&self) -> &sqlx::Pool<Self::DB> {
        &self.pool
    }

    async fn execute<'q>(
        &'q self,
        sql: String,
        arguments: <Self::DB as sqlx::Database>::Arguments<'q>,
    ) -> Result<impl crate::QueryResult> {
        let res = sqlx::query_with::<Self::DB, _>(&sql, arguments)
            .execute(&self.pool)
            .await?;

        Ok(PostgresQueryResult::new(res.rows_affected(), 0))
    }

    async fn fetch_one<'a, O>(
        &'a self,
        sql: String,
        arguments: <Self::DB as sqlx::Database>::Arguments<'a>,
    ) -> Result<O>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin,
    {
        sqlx::query_as_with::<Self::DB, O, _>(&sql, arguments)
            .fetch_one(&self.pool)
            .await
            .map_err(Into::into)
    }

    async fn fetch_all<'a, O>(
        &'a self,
        sql: String,
        arguments: <Self::DB as sqlx::Database>::Arguments<'a>,
    ) -> Result<Vec<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin,
    {
        sqlx::query_as_with::<Self::DB, O, _>(&sql, arguments)
            .fetch_all(&self.pool)
            .await
            .map_err(Into::into)
    }

    async fn insert<'a>(&'a self, statement: &'a Statement<Self::DB>) -> Result<impl QueryResult> {
        let (sql, arguments) = QueryBuilder::new(false).insert(statement);

        self
            .execute(sql, arguments)
            .await
    }

    async fn insert_as<'a, O>(&self, statement: &'a Statement<Self::DB>) -> Result<O>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin,
    {
        let (mut sql, arguments) = QueryBuilder::new(false).insert(statement);
        sql.push_str(" RETURNING *");

        sqlx::query_as_with::<Self::DB, O, _>(&sql, arguments)
            .fetch_one(&self.pool)
            .await
            .map_err(Into::into)
    }

    async fn update<'a>(&'a self, statement: &'a Statement<Self::DB>) -> Result<impl QueryResult> {
        let (sql, arguments) = QueryBuilder::new(false).update(statement);

        self
            .execute(sql, arguments)
            .await
    }

    async fn delete<'a>(&self, statement: &'a Statement<Self::DB>) -> Result<impl QueryResult> {
        let (sql, arguments) = QueryBuilder::new(false).delete(statement);

        self
            .execute(sql, arguments)
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

        sqlx::query_scalar_with::<Self::DB, i64, _>(&sql, arguments)
            .fetch_one(&self.pool)
            .await
            .map(|total| total)
            .map_err(Into::into)
    }

    async fn all<'a, O>(&self, statement: &'a Statement<Self::DB>) -> Result<Vec<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin,
    {
        let (sql, arguments) = QueryBuilder::new(false).query(statement);

        sqlx::query_as_with::<Self::DB, O, _>(&sql, arguments)
            .fetch_all(&self.pool)
            .await
            .map_err(Into::into)
    }

    async fn first<'a, O>(&self, statement: &'a Statement<Self::DB>) -> Result<O>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin,
    {
        let (sql, arguments) = QueryBuilder::new(false).query(statement);

        sqlx::query_as_with::<Self::DB, O, _>(&sql, arguments)
            .fetch_one(&self.pool)
            .await
            .map_err(Into::into)
    }

    async fn paginate<'a, O>(&self, statement: &'a Statement<Self::DB>) -> Result<Pagination<O>>
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