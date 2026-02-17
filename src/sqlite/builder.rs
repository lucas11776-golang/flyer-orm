use anyhow::{Ok, Result};

use crate::QueryBuilder;
use crate::query::QueryStatement;

#[derive(Default, Debug)]
pub(crate) struct Builder;

impl QueryBuilder for Builder {
    fn build(&self, statement: &QueryStatement) -> Result<String> {
        let mut query = vec![
            "SELECT".to_string(),
                format!("{}", Self::select(statement).as_str()),
            "FROM".to_string(),
                format!("{}", statement.table),
        ];

        if statement.join.len() != 0 {
            query.push(format!("{}", Self::join(statement).as_str()));
        }

        if statement.where_queries.len() != 0 {
            query.extend([
                "WHERE".to_string(),
                    format!("{}", Self::r#where(statement).as_str()),
            ]);
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
    fn join<'q>(statement: &'q QueryStatement) -> String {
        let mut conditions: Vec<String> = Vec::new();

        for join in &statement.join {
            match join.join_type {
                crate::query::JoinType::LeftJoin => conditions.push(format!("LEFT JOIN {} ON {} {} {}", join.table, join.column, join.operator, join.column_table)),
                crate::query::JoinType::RightJoin => conditions.push(format!("RIGHT JOIN {} ON {} {} {}", join.table, join.column, join.operator, join.column_table)),
                crate::query::JoinType::InnerJoin => conditions.push(format!("INNER JOIN {} ON {} {} {}", join.table, join.column, join.operator, join.column_table)),
                crate::query::JoinType::FullOuterJoin => conditions.push(format!("FULL OUTER JOIN {} ON {} {} {}", join.table, join.column, join.operator, join.column_table)),
                crate::query::JoinType::CrossJoin => conditions.push(format!("CROSS JOIN {} ON {} {} {}", join.table, join.column, join.operator, join.column_table)),
            }
        }

        return conditions.join(" ");
    }
}

impl Builder {
    fn r#where<'q>(statement: &'q QueryStatement) -> String {
        let mut conditions: Vec<String> = Vec::new();

        for where_query in &statement.where_queries {



            match &where_query.group {
                Some(group) => {

                },
                None => {
                    match where_query.operator.clone().unwrap().as_str().to_lowercase().as_str() {
                        "like" => conditions.push(format!("{} LIKE '%' || ? || '%'", where_query.column.clone().unwrap())),
                        _ => conditions.push(format!("{} {} ?", where_query.column.clone().unwrap(), where_query.operator.clone().unwrap())),
                    }
                },
            }


            
        }

        return conditions.join(" ");
    }
}

