use sqlx::{
    Database as SqlxDatabase, MySqlPool,
};

use crate::{
    Entity,
    Executor,
    Pagination,
    QueryResult,
    database::mysql::builder::QueryBuilder,
};

mod builder;

pub struct MySQLQueryResult {
    pub(crate) affected: u64,
    pub(crate) id: u64,
}

impl MySQLQueryResult {
    pub fn new(affected: u64, id: u64) -> Self {
        return Self {
            affected: affected,
            id: id,
        };
    }
}

impl QueryResult for MySQLQueryResult {
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

pub struct MySQL {
    pool: MySqlPool
}

impl MySQL {
    pub async fn new(url: impl Into<String>) -> Self {
        return Self {
            pool: MySqlPool::connect(&url.into())
                .await
                .unwrap()
        }
    }
}

impl Executor for MySQL {
    type DB = sqlx::MySql;

    async fn new(url: impl Into<String>) -> Self {
        return Self {
            pool: MySqlPool::connect(&url.into())
                .await
                .unwrap()
        }
    }
    
    fn from(pool: sqlx::Pool<Self::DB>) -> Self {
        return Self {
            pool: pool
        }
    }

    fn to_sql<'q>(&self, statement: &crate::Statement<Self::DB>) -> String {
        return QueryBuilder::new(true).to_sql(statement);
    }
    
    fn db(&self) -> &sqlx::Pool<Self::DB> {
        return &self.pool;
    }
    
    async fn execute<'c>(&self, sql: String, arguments: <Self::DB as sqlx::Database>::Arguments<'c>) -> crate::Result<impl crate::QueryResult> {
        return sqlx::query_with(&sql, arguments)
            .execute(&self.pool)
            .await
            .map_err(|err| err.into())
            .map(|result | MySQLQueryResult::new(result.rows_affected(), result.last_insert_id() as u64));
    }

    async fn fetch_one<'c, O>(&self, sql: String, arguments: <Self::DB as sqlx::Database>::Arguments<'c>) -> crate::Result<O>
    where
        O: crate::Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin
    {
        return sqlx::query_as_with::<Self::DB, O, _>(&sql, arguments)
            .fetch_one(&self.pool)
            .await
            .map_err(|err| err.into());
    }
    
    async fn fetch_all<'c, O>(&self, sql: String, arguments: <Self::DB as sqlx::Database>::Arguments<'c>) -> crate::Result<Vec<O>>
    where
        O: crate::Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin
    {
        return sqlx::query_as_with::<Self::DB, O, _>(&sql, arguments)
            .fetch_all(&self.pool)
            .await
            .map_err(|err| err.into());
    }
    
    async fn insert<'q>(&self, statement: &crate::Statement<Self::DB>) -> crate::Result<()> {
        let (sql, arguments) = QueryBuilder::new(false).insert(statement);
        self.execute(sql, arguments).await?;
        return Ok(());
    }
    
    async fn update<'q>(&self, _statement: &crate::Statement<Self::DB>) -> crate::Result<()> {
        todo!("Update is not yet implemented for MySQL")
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

        return sqlx::query_as_with::<Self::DB, Total, _>(&sql, arguments)
            .fetch_one(&self.pool)
            .await
            .map(|total| total.total as u64)
            .map_err(|err| err.into());
    }
    
    async fn delete<'q>(&self, _statement: &crate::Statement<Self::DB>) -> crate::Result<()> {
        todo!("Delete is not yet implemented for MySQL")
    }
    
    async fn insert_as<'q, O>(&self, _statement: &crate::Statement<Self::DB>) -> crate::Result<O>
    where
        O: crate::Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin {
        todo!("Insert as is not yet implemented for MySQL")
    }

    async fn all<O>(&self, statement: &crate::Statement<Self::DB>) -> crate::Result<Vec<O>>
    where
        O: crate::Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin {
        let (sql, arguments) = QueryBuilder::new(false).query(statement);

        return  sqlx::query_as_with::<Self::DB, O, _>(&sql, arguments)
            .fetch_all(&self.pool)
            .await
            .map_err(|err| err.into());
    }

    async fn first<O>(&self, statement: &crate::Statement<Self::DB>) -> crate::Result<O>
    where
        O: crate::Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin {
        let (sql, arguments) = QueryBuilder::new(false).query(statement);

        return  sqlx::query_as_with::<Self::DB, O, _>(&sql, arguments)
            .fetch_one(&self.pool)
            .await
            .map_err(|err| err.into());
    }

    async fn get<O>(&self, statement: &crate::Statement<Self::DB>) -> crate::Result<Vec<O>>
    where
        O: crate::Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin {
        return self.all(statement).await;
    }

    async fn paginate<O>(&self, statement: &crate::Statement<Self::DB>) -> crate::Result<crate::Pagination<O>>
    where
        O: crate::Entity + for<'r> sqlx::FromRow<'r, <Self::DB as sqlx::Database>::Row> + Send + Unpin {
        let items: Vec<O> = self
            .get(statement)
            .await
            .unwrap();

        let (sql, arguments) = QueryBuilder::new(false)
            .select(&vec!["COUNT(*) AS total".into()])
            .from(&statement.table)
            .joins(&statement.join)
            .conditions(&statement.conditions, true)
            .group_by(&statement.group_by)
            .having(&statement.having)
            .compile();

        let total =  sqlx::query_as_with::<Self::DB, Total, _>(&sql, arguments)
            .fetch_one(&self.pool)
            .await
            .unwrap();

        // TODO: need to get limit, page as u64 in Bindable<>
        return Ok(Pagination {
            total: total.total as u64,
            page: 1,
            per_page: 10,
            items: items,
        });
    }
}
