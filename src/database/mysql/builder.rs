use std::{fmt::Write, mem};

use crate::{
    database::Builder,
    query::{Having, Join, Limit, Offset, OrderValue, Statement},
    types::{ArgsAsRef, Bindable, WhereClause},
};

pub struct QueryBuilder<'c, DB: sqlx::Database> {
    arguments: ArgsAsRef<'c, DB>,
    sql: String,
    dry_run: bool,
}

impl <'c, DB: sqlx::Database>Builder<'c, DB> for QueryBuilder<'c, DB> {
    fn new(dry_run: bool) -> Self {
        Self {
            arguments: Default::default(),
            sql: String::new(),
            dry_run,
        }
    }

    #[inline]
    fn push_placeholder(&mut self, prefix: &str, suffix: &str) {
        let _ = write!(self.sql, "{}?{}", prefix, suffix);
    }

    fn to_sql(&mut self, statement: &'c Statement<DB>) -> String {
        self.query(statement).0
    }

    fn insert(&mut self, statement: &'c Statement<DB>) -> (String, ArgsAsRef<'c, DB>) {
        self.sql.push_str("INSERT INTO ");
        self.sql.push_str(&statement.table);

        if statement.values.is_empty() {
            self.sql.push_str(" () VALUES ()");
            return self.compile();
        }

        self.sql.push_str(" (");
        for (idx, (col, _)) in statement.values.iter().enumerate() {
            if idx > 0 {
                self.sql.push_str(", ");
            }
            self.sql.push_str(col);
        }

        self.sql.push_str(") VALUES (");
        for (idx, (_, val)) in statement.values.iter().enumerate() {
            if idx > 0 {
                self.sql.push_str(", ");
            }
            self.push_placeholder("", "");
            self.bind_to(val);
        }
        self.sql.push(')');

        self.compile()
    }

    fn update(&mut self, statement: &'c Statement<DB>) -> (String, ArgsAsRef<'c, DB>) {
        self.sql.push_str("UPDATE ");
        self.sql.push_str(&statement.table);
        self.sql.push_str(" SET ");

        for (idx, (col, val)) in statement.values.iter().enumerate() {
            if idx > 0 {
                self.sql.push_str(", ");
            }
            let _ = write!(self.sql, "{} = ", col);
            self.push_placeholder("", "");
            self.bind_to(val);
        }

        self.conditions(&statement.conditions, true);

        self.compile()
    }

    fn delete(&mut self, statement: &'c Statement<DB>) -> (String, ArgsAsRef<'c, DB>) {
        self.sql.push_str("DELETE FROM ");
        self.sql.push_str(&statement.table);

        self.conditions(&statement.conditions, true);

        self.compile()
    }

    fn query(&mut self, statement: &'c Statement<DB>) -> (String, ArgsAsRef<'c, DB>) {
        self.select(&statement.fields)
            .from(&statement.table)
            .joins(&statement.join)
            .conditions(&statement.conditions, true)
            .group_by(&statement.group_by)
            .having(&statement.having)
            .order_by(&statement.order_by)
            .limit(&statement.limit)
            .offset(&statement.offset)
            .compile()
    }

    fn select(&mut self, fields: &[String]) -> &mut Self {
        self.sql.push_str("SELECT ");

        if fields.is_empty() {
            self.sql.push('*');
            return self;
        }

        for (idx, field) in fields.iter().enumerate() {
            if idx > 0 {
                self.sql.push_str(", ");
            }
            self.sql.push_str(field);
        }

        self
    }

    fn from(&mut self, table: &str) -> &mut Self {
        self.sql.push_str(" FROM ");
        self.sql.push_str(table);

        self
    }

    fn joins(&mut self, joins: &'c [Join]) -> &mut Self {
        for join in joins {
            let _ = write!(
                self.sql,
                " {} {} ON {} {} {}",
                join.join_type.to_string(),
                join.table,
                join.column,
                join.operator,
                join.column_table
            );
        }

        self
    }

    fn conditions(&mut self, conditions: &'c [WhereClause<DB>], is_root: bool) -> &mut Self {
        if conditions.is_empty() {
            return self;
        }

        if is_root {
            self.sql.push_str(" WHERE ");
        }

        for (idx, cond) in conditions.iter().enumerate() {
            if idx > 0 {
                let _ = write!(self.sql, " {} ", cond.connector().to_string());
            }

            match cond {
                WhereClause::Clause {
                    column,
                    operator,
                    value,
                    ..
                } => {
                    let op = operator.trim().to_lowercase();

                    match op.as_str() {
                        "like" | "ilike" | "not like" | "not ilike" => {
                            let sql_op = if op.contains("not") {
                                "NOT LIKE"
                            } else {
                                "LIKE"
                            };
                            let _ = write!(self.sql, "{} {} CONCAT('%', ", column, sql_op);
                            self.push_placeholder("", "");
                            self.sql.push_str(", '%')");
                        }
                        _ => {
                            let _ = write!(self.sql, "{} {} ", column, operator);
                            self.push_placeholder("", "");
                        }
                    }

                    self.bind_to(value);
                }

                WhereClause::NullCheck {
                    column, is_null, ..
                } => {
                    let op = if *is_null { "IS NULL" } else { "IS NOT NULL" };
                    let _ = write!(self.sql, "{} {}", column, op);
                }

                WhereClause::In {
                    column,
                    negated,
                    values,
                    ..
                } => {
                    if values.is_empty() {
                        self.sql.push_str("1 = 0");
                        continue;
                    }

                    let op = if *negated { "NOT IN" } else { "IN" };
                    let _ = write!(self.sql, "{} {} (", column, op);

                    for (v_idx, val) in values.iter().enumerate() {
                        if v_idx > 0 {
                            self.sql.push_str(", ");
                        }
                        self.push_placeholder("", "");
                        self.bind_to(val);
                    }

                    self.sql.push(')');
                }

                WhereClause::Between {
                    column,
                    negated,
                    low,
                    high,
                    ..
                } => {
                    let op = if *negated { "NOT BETWEEN" } else { "BETWEEN" };
                    let _ = write!(self.sql, "{} {} ", column, op);

                    self.push_placeholder("", "");
                    self.bind_to(low);

                    self.sql.push_str(" AND ");

                    self.push_placeholder("", "");
                    self.bind_to(high);
                }

                WhereClause::Group {
                    conditions: sub_conds,
                    ..
                } => {
                    self.sql.push('(');
                    self.conditions(sub_conds, false);
                    self.sql.push(')');
                }
            }
        }

        self
    }

    fn group_by(&mut self, group_by: &'c Option<String>) -> &mut Self {
        if let Some(column) = group_by {
            self.sql.push_str(" GROUP BY ");
            self.sql.push_str(column);
        }

        self
    }

    fn having(&mut self, having: &'c [Having<DB>]) -> &mut Self {
        if having.is_empty() {
            return self;
        }

        self.sql.push_str(" HAVING ");

        for (idx, h) in having.iter().enumerate() {
            if idx > 0 {
                let _ = write!(self.sql, " {} ", h.connector.to_string());
            }

            let _ = write!(self.sql, "{} {} ", h.column, h.operator);

            self.push_placeholder("", "");
            self.bind_to(&h.value);
        }

        self
    }

    fn order_by(&mut self, order_by: &[OrderValue]) -> &mut Self {
        if order_by.is_empty() {
            return self;
        }

        self.sql.push_str(" ORDER BY ");

        for (idx, o) in order_by.iter().enumerate() {
            if idx > 0 {
                self.sql.push_str(", ");
            }
            let _ = write!(self.sql, "{} {}", o.column, o.order.to_string());
        }

        self
    }

    fn limit(&mut self, limit: &'c Option<Limit<DB>>) -> &mut Self {
        if let Some(limit) = limit {
            self.bind_to(&limit.value);
            self.sql.push_str(" LIMIT ");
            self.push_placeholder("", "");
        }

        self
    }

    fn offset(&mut self, offset: &'c Option<Offset<DB>>) -> &mut Self {
        if let Some(offset) = offset {
            self.bind_to(&offset.value);
            self.sql.push_str(" OFFSET ");
            self.push_placeholder("", "");
        }

        self
    }

    fn compile(&mut self) -> (String, ArgsAsRef<'c, DB>) {
        (mem::take(&mut self.sql), mem::take(&mut self.arguments))
    }

    fn bind_to(&mut self, value: &'c Box<dyn Bindable<DB>>) {
        if !self.dry_run {
            self.arguments.push(value);
        }
    }
}
