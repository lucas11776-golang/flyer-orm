use anyhow::{Ok, Result};

use crate::QueryBuilder;
use crate::query::QueryStatement;

pub(crate) struct Builder;

impl QueryBuilder for Builder {
    fn build<'q>(statement: &'q QueryStatement) -> Result<String> {
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
    fn select<'q>(statement: &'q QueryStatement) -> String {
        if statement.select.len() == 0 {
            return "*".to_string();
        }

        return statement.select.join(", ");
    }
}

