use std::vec;

use anyhow::{Ok, Result};

use crate::query::{QueryBuilder, Query};
use crate::query::logic::{Condition, JoinType};

#[derive(Debug)]
pub(crate) struct Builder<'q> {
    statement: &'q Query,
}

impl <'q>QueryBuilder<'q> for Builder<'q> {
    fn new(statement: &'q Query) -> Self {
        return Self {
            statement: statement
        };
    }

    fn query(&self) -> Result<String> {
        let mut sql = vec![
            "SELECT".to_string(),
                format!("{}", self.select().unwrap().as_str()),
            "FROM".to_string(),
                format!("{}", self.statement.table),
        ];

        if self.statement.join.len() != 0 {
            sql.push(format!("{}", self.join().unwrap().as_str()));
        }

        if self.statement.where_queries.len() != 0 {
            sql.extend([
                "WHERE".to_string(),
                    format!("{}", self.r#where().unwrap().as_str()),
            ]);
        }

        // TODO: use query logic builder
        // sql.push(self.select().unwrap());   // SELECT
        // sql.push(self.from().unwrap());     // FROM
        // sql.push(self.join().unwrap());     // JOIN
        // sql.push(self.r#where().unwrap());  // WHERE
        sql.push(self.group_by().unwrap()); // GROUP BY
        // sql.push(self.having().unwrap());   // GROUP BY
        // sql.push(self.order_by().unwrap()); // GROUP BY
        // sql.push(self.limit().unwrap());    // GROUP BY

        return Ok(String::from(sql.join(" ").trim()));
    }

    fn insert(&self) -> Result<String> {
        let columns = self.statement.columns.clone().unwrap();

        return Ok(format!(
            "INSERT INTO {} ({}) VALUES ({});",
            self.statement.table,
            columns.join(", "),
            std::iter::repeat("?").take(columns.len()).collect::<Vec<_>>().join(", ")
        ));
    }
    
    fn update(&self) -> Result<String> {
        let mut sql = vec![
            String::from(format!("UPDATE {}", self.statement.table)),
            format!("SET {}", self.statement
                .columns
                .clone()
                .unwrap()
                .iter()
                .map(|f| format!("{} = ?", f))
                .collect::<Vec<_>>()
                .join(", ")
            )
        ];

        if self.statement.where_queries.len() != 0 {
            sql.extend([
                "WHERE".to_string(),
                    format!("{}", self.r#where().unwrap().as_str()),
            ]);
        }

        if self.statement.group_by.is_some() {

        }

        return Ok(sql.join(" "));
    }
    
    fn delete(&self) -> Result<String> {
        let mut sql = vec![String::from(format!("DELETE FROM {}", self.statement.table))];

        if self.statement.where_queries.len() != 0 {
            sql.extend([
                "WHERE".to_string(),
                    format!("{}", self.r#where().unwrap().as_str()),
            ]);
        }

        return Ok(sql.join(" "));
    }

    fn select(&self) -> Result<String> {
        if self.statement.select.len() == 0 {
            return Ok(String::from("*"));
        }

        return Ok(self.statement.select.join(", "));
    }

    fn join(&self) -> Result<String> {
        let mut conditions: Vec<String> = Vec::new();

        for join in &self.statement.join {
            match join.join_type {
                JoinType::LeftJoin => conditions.push(format!("LEFT JOIN {} ON {} {} {}", join.table, join.column, join.operator, join.column_table)),
                JoinType::RightJoin => conditions.push(format!("RIGHT JOIN {} ON {} {} {}", join.table, join.column, join.operator, join.column_table)),
                JoinType::InnerJoin => conditions.push(format!("INNER JOIN {} ON {} {} {}", join.table, join.column, join.operator, join.column_table)),
                JoinType::FullOuterJoin => conditions.push(format!("FULL OUTER JOIN {} ON {} {} {}", join.table, join.column, join.operator, join.column_table)),
                JoinType::CrossJoin => conditions.push(format!("CROSS JOIN {} ON {} {} {}", join.table, join.column, join.operator, join.column_table)),
            }
        }

        return Ok(conditions.join(" "));
    }

    fn r#where(&self) -> Result<String> {
        let mut conditions: Vec<String> = Vec::new();

        for where_query in &self.statement.where_queries {
            if let Some(position) = &where_query.position {
                match position {
                    Condition::AND => conditions.push(String::from("AND")),
                    Condition::OR => conditions.push(String::from("OR")),
                }
            }

            match &where_query.group {
                Some(_group) => {

                },
                None => {
                    match where_query.operator.clone().unwrap().as_str().to_lowercase().as_str() {
                        "like" => conditions.push(format!("{} LIKE '%' || ? || '%'", where_query.column.clone().unwrap())),
                        _ => conditions.push(format!("{} {} ?", where_query.column.clone().unwrap(), where_query.operator.clone().unwrap())),
                    }
                },
            }
        }

        return Ok(conditions.join(" "));
    }
    
    fn group_by(&self) -> Result<String> {
        return Ok(self.statement.group_by.clone().map(|c| format!("GROUP BY {}", c)).or(Some(String::from(""))).unwrap());
    }
    
}