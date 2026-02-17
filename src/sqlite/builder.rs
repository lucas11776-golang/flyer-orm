use anyhow::{Ok, Result};

use crate::QueryBuilder;
use crate::query::QueryStatement;

#[derive(Default)]
pub(crate) struct Builder;

impl QueryBuilder for Builder {
    fn build(&self, statement: &QueryStatement) -> Result<String> {
        let query = [
            "SELECT".to_string(),
                format!("{}", Self::select(statement).as_str()),
            "FROM".to_string(),
                format!("{}", statement.table),
        ];

        if statement.where_queries.len() != 0 {

        }

        return Ok(query.join(" "));
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

impl Builder {
    fn r#where<'q>(statement: &'q QueryStatement) -> String {
        let conditions: Vec<String> = Vec::new();


        return conditions.join(" ");
    }
}

