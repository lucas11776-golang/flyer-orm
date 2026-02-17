use anyhow::{Ok, Result};

use crate::QueryBuilder;
use crate::query::Statement;

pub(crate) struct Builder;

impl <DB: sqlx::Database>QueryBuilder<DB> for Builder {
    fn build<'q>(statement: &'q Statement<'q, DB>) -> Result<String> {
        let query = [
            "SELECT".to_string(),
                format!("  {}", Self::select(statement).as_str()),
            "FROM".to_string(),
                format!("  {}", statement.table),
        ];

        return Ok(query.join("\r\n"));
    }
    
}

impl Builder {
    fn select<'q, DB: sqlx::Database>(statement: &'q Statement<'q, DB>) -> String {
        if statement.select.len() == 0 {
            return "*".to_string();
        }

        return statement.select.join(", ");
    }
}

