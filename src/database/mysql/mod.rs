use crate::{
    Entity,
    Executor,
    MySqlPool,
    Pagination,
    QueryResult,
    Result,
    database::mysql::builder::QueryBuilder,
    query::Statement,
};

mod builder;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MySQLQueryResult {
    pub(crate) affected: u64,
    pub(crate) id: u64,
}

impl MySQLQueryResult {
    pub fn new(affected: u64, id: u64) -> Self {
        Self { affected, id }
    }
}

impl QueryResult for MySQLQueryResult {
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

    async fn new(url: impl Into<String>) -> Self {
        Self {
            pool: MySqlPool::connect(&url.into())
                .await
                .unwrap(),
        }
    }

    fn from(pool: sqlx::Pool<Self::DB>) -> Self {
        Self { pool }
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

        Ok(MySQLQueryResult::new(
            result.rows_affected(),
            result.last_insert_id(),
        ))
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

    async fn insert_as<'a, O>(&'a self, statement: &'a Statement<Self::DB>) -> Result<O>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin,
    {
        let (sql, arguments) = QueryBuilder::new(false).insert(statement);

        let res = sqlx::query_with::<Self::DB, _>(&sql, arguments)
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
            .execute(sql, arguments)
            .await
    }

    async fn delete<'a>(&'a self, statement: &'a Statement<Self::DB>) -> Result<impl QueryResult> {
        let (sql, arguments) = QueryBuilder::new(false).delete(statement);

        self
            .execute(sql, arguments)
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

        sqlx::query_scalar_with::<Self::DB, i64, _>(&sql, arguments)
            .fetch_one(&self.pool)
            .await
            .map(|total| total)
            .map_err(Into::into)
    }

    async fn first<'a, O>(&'a self, statement: &'a Statement<Self::DB>) -> Result<O>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin,
    {
        let (sql, arguments) = QueryBuilder::new(false).query(statement);

        self
            .fetch_one(sql, arguments)
            .await
    }

    async fn all<'a, O>(&'a self, statement: &'a Statement<Self::DB>) -> Result<Vec<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin,
    {
        let (sql, arguments) = QueryBuilder::new(false).query(statement);

        self
            .fetch_all(sql, arguments)
            .await
    }

    async fn paginate<'a, O>(&'a self, statement: &'a Statement<Self::DB>) -> Result<Pagination<O>>
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