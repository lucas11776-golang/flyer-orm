use std::vec;

use crate::query::{QueryBuilder};
use crate::query::logic::{JoinType, SqlQuery};

#[derive(Debug)]
pub(crate) struct Builder<'q> {
    statement: &'q SqlQuery,
}

impl <'q>QueryBuilder<'q> for Builder<'q> {
    fn new(statement: &'q SqlQuery) -> Self where Self: Sized {
        return Self { statement: statement };
    }

    fn query(&mut self) -> String {
        return format!(
            "{};",
            vec![self.select(), self.from(), self.join(), self.r#where(), self.group_by(), self.having(), self.order_by(), self.limit()]
                .iter()
                .filter(|q| !q.is_empty())
                .map(|q| String::from(q))
                .collect::<Vec<String>>()
                .join(" ")
        );
    }

    fn insert(&mut self) -> String {
        return format!(
            "INSERT INTO {} ({}) VALUES ({});",
            self.statement.table,
            self.statement.columns.clone().join(", "),
            std::iter::repeat("?").take(self.statement.columns.len()).collect::<Vec<_>>().join(", ")
        );
    }

    fn update(&mut self) -> String {
        return format!(
            "UPDATE {} SET {} {};",
            self.statement.table,
            self.statement.columns.iter().map(|c| format!("{} = ?", c)).collect::<Vec<_>>().join(" "),
            self.r#where()
        );
    }

    fn delete(&mut self) -> String {
        return String::from(
            format!(
                "DELETE FROM {} {};",
                self.statement.table,
                self.r#where()
            ).trim()
        );
    }
}

impl <'q>Builder<'q> {
    fn select(&self) -> String {
        if self.statement.select.is_empty() {
            return format!("SELECT *");
        }

        return format!("SELECT {}", self.statement.select.join(", "));
    }

    fn from(&self) -> String {
        return format!("FROM {}", self.statement.table);
    }

    fn join(&self) -> String {
        return self.statement
            .join
            .iter()
            .map(|join| {
                match join.join_type {
                    JoinType::LeftJoin      => format!("LEFT JOIN {} ON {} {} {}", join.table, join.column, join.operator, join.column_table),
                    JoinType::RightJoin     => format!("RIGHT JOIN {} ON {} {} {}", join.table, join.column, join.operator, join.column_table),
                    JoinType::InnerJoin     => format!("INNER JOIN {} ON {} {} {}", join.table, join.column, join.operator, join.column_table),
                    JoinType::FullOuterJoin => format!("FULL OUTER JOIN {} ON {} {} {}", join.table, join.column, join.operator, join.column_table),
                    JoinType::CrossJoin     => format!("CROSS JOIN {} ON {} {} {}", join.table, join.column, join.operator, join.column_table),
                }
            })
            .collect::<Vec<String>>()
            .join(" ");
    }

    // TODO: refactor
    fn r#where(&self) -> String {
        if self.statement.where_queries.len() == 0 {
            return String::from("");
        }

        return format!(
            "WHERE {}",
            self.statement.where_queries.iter().map(|clause| {
                return String::from(
                    format!(
                        "{} {}",
                        clause.condition.clone().map(|t| t.to_string()).or(Some(String::from(""))).unwrap(),
                        match &clause.group {
                            Some(_group) => String::from(""), // TODO: implement will need Where to hold Encode/Value for order.
                            None => match clause.operator.clone().unwrap().to_ascii_lowercase().as_str() {
                                "like" => format!("{} LIKE '%' || ? || '%'", clause.column.clone().unwrap()),
                                _ => format!("{} {} ?",  clause.column.clone().unwrap(), clause.operator.clone().unwrap()),
                            }
                        }
                    ).trim()
                );
            }).collect::<Vec<_>>().join(" ")
        );
    }

    fn group_by(&self) -> String {
        return self.statement
            .group_by
            .clone()
            .map(|c| format!("GROUP BY {}", c))
            .or(Some(String::from("")))
            .unwrap();
    }

    fn having(&self) -> String {
        return self.statement
            .having
            .clone()
            .map(|t| format!("{} {} ?", t.column, t.operator))
            .or(Some(String::new()))
            .unwrap();
    }

    fn order_by(&self) -> String {
        return self.statement
            .order_by
            .clone()
            .map(|ol| {
                format!(
                    "ORDER BY {}",
                    ol.iter().map(|o| format!("{} {}", o.column, o.order.to_string())).collect::<Vec<_>>().join(", ")
                )
            })
            .or(Some(String::new()))
            .unwrap();
    }

    fn limit(&self) -> String {
        return self.statement.limit.map(|_limit| {
            return format!("LIMIT ?{}", self.statement.page.map(|_t| {
                return format!(" OFFSET ?");
            }).unwrap_or(String::new()));
        }).unwrap_or(String::new());
    }    
}
