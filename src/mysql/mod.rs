mod builder;

use anyhow::Result;
use sqlx::{FromRow, MySql, MySqlPool, Pool};

use crate::{Executor, query::{Pagination, Statement}};

pub struct MySQL {
    db: Pool<MySql>,
}

impl MySQL {
    pub async fn connect(url: &str) -> Result<Self> {
        return Ok(Self {
            db: MySqlPool::connect(url).await.unwrap()
        });
    }
}

// impl Executor for MySQL {
//     async fn get<T: sqlx::Database, O>(&self, statement: Statement) -> Result<Vec<O>>
//     where
//         O: for<'r> FromRow<'r, <T as sqlx::Database>::Row> + Send + Unpin
//     {
//         todo!()
//     }

//     async fn pagination<T: sqlx::Database, O>(&self, statement: Statement) -> Result<Pagination<O>>
//     where
//         O: for<'r> FromRow<'r, <T as sqlx::Database>::Row> + Send + Unpin
//     {
//         todo!()
//     }
// }