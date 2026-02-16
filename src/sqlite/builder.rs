use anyhow::Result;

use crate::QueryBuilder;
use crate::query::Statement;

pub(crate) struct Builder;

impl <'q, DB: sqlx::Database>QueryBuilder<'q, DB> for Builder {
    fn build(statement: &Statement<'q, DB>) -> Result<String> {

        println!("\r\n\r\n ------------ RUNNING BUILDER ------------ \r\n\r\n {:?} \r\n\r\n\r\n\r\n", statement.where_queries);

        return Ok(format!("SELECT * FROM `{}`", statement.table));
    }
}

