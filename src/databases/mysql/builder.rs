use std::vec;

use crate::{
    query::{QueryBuilder},
    types::{JoinType, SQL}
};

#[derive(Debug)]
pub(crate) struct _Builder<'q> {
    statement: &'q SQL,
}

impl <'q>QueryBuilder<'q> for _Builder<'q> {
    fn new(statement: &'q SQL) -> Self where Self: Sized {
        return Self { statement: statement };
    }

    fn query(&mut self) -> String {
        return format!(
            "{};",
            vec![self._select(), self._from(), self._join(), self._where(), self._group_by(), self._having(), self._order_by(), self._limit()]
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
            self.statement.select.clone().join(", "),
            std::iter::repeat("?").take(self.statement.select.len()).collect::<Vec<_>>().join(", ")
        );
    }

    fn update(&mut self) -> String {
        return format!(
            "UPDATE {} SET {} {};",
            self.statement.table,
            self.statement.select.iter().map(|c| format!("{} = ?", c)).collect::<Vec<_>>().join(" "),
            self._where()
        );
    }

    fn delete(&mut self) -> String {
        return String::from(
            format!(
                "DELETE FROM {} {};",
                self.statement.table,
                self._where()
            ).trim()
        );
    }
}

impl <'q>_Builder<'q> {
    fn _select(&self) -> String {
        if self.statement.select.is_empty() {
            return format!("SELECT *");
        }

        return format!("SELECT {}", self.statement.select.join(", "));
    }

    fn _from(&self) -> String {
        return format!("FROM {}", self.statement.table);
    }

    fn _join(&self) -> String {
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
    fn _where(&self) -> String {
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

    fn _group_by(&self) -> String {
        return self.statement
            .group_by
            .clone()
            .map(|c| format!("GROUP BY {}", c))
            .or(Some(String::from("")))
            .unwrap();
    }

    fn _having(&self) -> String {
        return self.statement
            .having
            .clone()
            .map(|t| format!("{} {} ?", t.column, t.operator))
            .or(Some(String::new()))
            .unwrap();
    }

    fn _order_by(&self) -> String {
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

    fn _limit(&self) -> String {
        return String::new();
    }    
}
