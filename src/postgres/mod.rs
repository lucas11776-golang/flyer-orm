use sqlx::{
    Database as SqlxDatabase,
    PgPool,
};

use crate::{Entity, Executor, Pagination, postgres::builder::QueryBuilder};

mod builder;

#[derive(crate::Entity)]
struct Total {
    pub total: i64,
}

pub struct Postgres {
    pool: PgPool
}

impl Postgres {
    pub async fn new(url: impl Into<String>) -> Self {
        return Self {
            pool: PgPool::connect(&url.into())
                .await
                .unwrap()
        }
    }
}

impl Executor for Postgres {
    type DB = sqlx::Postgres;
    
    fn to_sql<'q>(&self, statement: &crate::Statement<Self::DB>) -> String {
        let (sql, _) = QueryBuilder::new(true).query(statement);

        return sql; 
    }

    async fn insert<'q>(&self) -> crate::Result<()> {
        todo!()
    }

    async fn all<O>(&self, statement: &crate::Statement<Self::DB>) -> crate::Result<Vec<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as SqlxDatabase>::Row> + Send + Unpin
    {
        let (sql, arguments) = QueryBuilder::new(false).query(statement);

        return  sqlx::query_as_with::<Self::DB, O, _>(&sql, arguments)
            .fetch_all(&self.pool)
            .await
            .map_err(|err| err.into());
    }

    async fn first<O>(&self, statement: &crate::Statement<Self::DB>) -> crate::Result<O>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as SqlxDatabase>::Row> + Send + Unpin
    {
        let (sql, arguments) = QueryBuilder::new(false).query(statement);

        return  sqlx::query_as_with::<Self::DB, O, _>(&sql, arguments)
            .fetch_one(&self.pool)
            .await
            .map_err(|err| err.into());
    }
    
    async fn get<O>(&self, statement: &crate::Statement<Self::DB>) -> crate::Result<Vec<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as SqlxDatabase>::Row> + Send + Unpin, 
    {
        return self.all(statement).await;
    }
    
    async fn paginate<O>(&self, statement: &crate::Statement<Self::DB>) -> crate::Result<crate::Pagination<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as SqlxDatabase>::Row> + Send + Unpin
    {
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
