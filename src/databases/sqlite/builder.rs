use anyhow::{Ok, Result};

use crate::query::{QueryBuilder};
use crate::query::logic::{JoinType, SqlQuery};

#[derive(Debug)]
pub(crate) struct Builder<'q> {
    statement: &'q SqlQuery,
}

impl <'q>QueryBuilder<'q> for Builder<'q> {
    fn new(statement: &'q SqlQuery) -> Self where Self: Sized {
        return Self {
            statement: statement
        };
    }

    fn query(&self) -> Result<String> {
        let mut sql = Vec::new();

        sql.push(self.select().unwrap());   // SELECT
        sql.push(self.from().unwrap());     // FROM
        sql.push(self.join().unwrap());     // JOIN
        sql.push(self.r#where().unwrap());  // WHERE
        sql.push(self.group_by().unwrap()); // GROUP BY
        sql.push(self.having().unwrap());   // HAVING
        sql.push(self.order_by().unwrap()); // ORDER BY
        sql.push(self.limit().unwrap());    // LIMIT
        
        return Ok(format!("{};", sql.iter().filter(|q| !q.is_empty()).map(|q| String::from(q)).collect::<Vec<String>>().join(" ")));
    }

    fn insert(&self) -> Result<String> {
        return Ok(format!(
            "INSERT INTO {} ({}) VALUES ({});",
            self.statement.table,
            self.statement.columns.clone().join(", "),
            std::iter::repeat("?").take(self.statement.columns.len()).collect::<Vec<_>>().join(", ")
        ));
    }

    fn update(&self) -> Result<String> {
        return Ok(
            format!(
                "UPDATE {} SET {} {};",
                self.statement.table,
                self.statement.columns.iter().map(|c| format!("{} = ?", c)).collect::<Vec<_>>().join(" "),
                self.r#where().unwrap()
            )
        );
    }

    fn delete(&self) -> Result<String> {
        return Ok(
            String::from(
                format!(
                    "DELETE FROM {} {};",
                    self.statement.table,
                    self.r#where().unwrap()
                ).trim()
            )
        );
    }

    fn select(&self) -> Result<String> {
        if self.statement.select.is_empty() {
            return Ok(format!("SELECT *"));
        }

        return Ok(format!("SELECT {}", self.statement.select.join(", ")));
    }

    fn from(&self) -> Result<String> {
        return Ok(format!("FROM {}", self.statement.table));
    }

    fn join(&self) -> Result<String> {
        return Ok(
            self.statement.join.iter().map(|join| {
                match join.join_type {
                    JoinType::LeftJoin => format!("LEFT JOIN {} ON {} {} {}", join.table, join.column, join.operator, join.column_table),
                    JoinType::RightJoin => format!("RIGHT JOIN {} ON {} {} {}", join.table, join.column, join.operator, join.column_table),
                    JoinType::InnerJoin => format!("INNER JOIN {} ON {} {} {}", join.table, join.column, join.operator, join.column_table),
                    JoinType::FullOuterJoin => format!("FULL OUTER JOIN {} ON {} {} {}", join.table, join.column, join.operator, join.column_table),
                    JoinType::CrossJoin => format!("CROSS JOIN {} ON {} {} {}", join.table, join.column, join.operator, join.column_table),
                }
            }).collect::<Vec<String>>().join(" ")
        );
    }

    // TODO: refactor
    fn r#where(&self) -> Result<String> {
        if self.statement.where_queries.len() == 0 {
            return Ok(String::from(""));
        }

        return Ok(
            format!(
                "WHERE {}",
                self.statement.where_queries.iter().map(|where_query| {
                    return String::from(
                        format!(
                            "{} {}",
                            where_query.condition.clone().map(|t| t.to_string()).or(Some(String::from(""))).unwrap(),
                            match &where_query.group {
                                Some(_group) => String::from(""), // TODO: implement will need Where to hold Encode/Value for order.
                                None => match where_query.operator.clone().unwrap().as_str().to_lowercase().as_str() {
                                    "like" => format!("{} LIKE '%' || ? || '%'", where_query.column.clone().unwrap()),
                                    _ => format!("{} {} ?",  where_query.column.clone().unwrap(), where_query.operator.clone().unwrap()),
                                }
                            }
                        ).trim()
                    );
                }).collect::<Vec<_>>().join(" ")
            )
        );
    }

    fn group_by(&self) -> Result<String> {
        return Ok(
            self.statement
                .group_by
                .clone()
                .map(|c| format!("GROUP BY {}", c))
                .or(Some(String::from("")))
                .unwrap()
        );
    }

    fn having(&self) -> Result<String> {
        // todo!()
        return Ok(String::new());
    }

    fn order_by(&self) -> Result<String> {
        // todo!()
        return Ok(String::new());
    }

    fn limit(&self) -> Result<String> {
        // todo!()
        return Ok(String::new());
    }    
}