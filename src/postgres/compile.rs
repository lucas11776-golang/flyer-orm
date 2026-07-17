use std::fmt::Write;

use crate::{Statement, WhereClause};

pub struct Builder<'c, DB: sqlx::Database> {
    statement: &'c Statement<DB>,
    arguments: <DB as sqlx::Database>::Arguments<'c>,
    params_index: i32,
    sql: String,
}

impl<'c, DB: sqlx::Database> Builder<'c, DB> {
    pub fn new(statement: &'c Statement<DB>) -> Self {
        Self {
            statement,
            arguments: Default::default(),
            params_index: 0,
            sql: String::new(),
        }
    }

    #[inline]
    fn push_placeholder(&mut self) {
        self.params_index += 1;
        let _ = write!(self.sql, "${}", self.params_index);
    }

    pub fn query(mut self) -> (String, <DB as sqlx::Database>::Arguments<'c>) {
        // Copy the statement reference out so it doesn't contentiously lock `self`
        let stmt = self.statement;

        self.sql.push_str("SELECT ");
        self.write_select();
        
        self.sql.push_str(" FROM ");
        self.sql.push_str(&stmt.table);

        self.write_joins();
        self.write_conditions(&stmt.conditions, true);
        self.write_group_by();
        self.write_having();
        self.write_order_by();
        self.write_limit();
        self.write_offset();

        println!("\r\n\r\n\r\n\r\n ToSQL: {}", self.sql);

        // Decompose the struct to return ownership of our final values
        (self.sql, self.arguments)
    }

    fn write_select(&mut self) {
        let stmt = self.statement;
        if stmt.fields.is_empty() {
            self.sql.push('*');
        } else {
            for (idx, field) in stmt.fields.iter().enumerate() {
                if idx > 0 {
                    self.sql.push_str(", ");
                }
                self.sql.push_str(field);
            }
        }
    }

    // 

    fn write_joins(&mut self) {
        let stmt = self.statement;
        for join in &stmt.join {
            self.sql.push(' ');
            self.sql.push_str(&join.join_type.to_string());
            self.sql.push(' ');
            self.sql.push_str(&join.table);
            self.sql.push_str(" ON ");
            self.sql.push_str(&join.column);
            self.sql.push(' ');
            self.sql.push_str(&join.operator);
            self.sql.push(' ');
            self.sql.push_str(&join.column_table);
        }
    }

    fn write_conditions(&mut self, conditions: &[WhereClause<DB>], is_root: bool) {
        if conditions.is_empty() {
            return;
        }

        if is_root {
            self.sql.push_str(" WHERE ");
        }

        for (idx, cond) in conditions.iter().enumerate() {
            if idx > 0 {
                self.sql.push(' ');
                self.sql.push_str(&cond.connector().to_string());
                self.sql.push(' ');
            }

            match cond {
                WhereClause::Clause {
                    column,
                    operator,
                    value,
                    ..
                } => {
                    self.sql.push_str(column);
                    self.sql.push(' ');
                    self.sql.push_str(operator);
                    self.sql.push(' ');
                    self.push_placeholder();
                    value.bind_to(&mut self.arguments).unwrap();
                }
                WhereClause::Group { conditions: sub_conds, .. } => {
                    self.sql.push('(');
                    self.write_conditions(sub_conds, false);
                    self.sql.push(')');
                }
            }
        }
    }

    fn write_group_by(&mut self) {
        if let Some(column) = &self.statement.group_by {
            self.sql.push_str(" GROUP BY ");
            self.sql.push_str(column);
        }
    }

    fn write_having(&mut self) {
        let stmt = self.statement;
        if stmt.having.is_empty() {
            return;
        }

        self.sql.push_str(" HAVING ");

        for (idx, h) in stmt.having.iter().enumerate() {
            if idx > 0 {
                self.sql.push(' ');
                self.sql.push_str(&h.connector.to_string());
                self.sql.push(' ');
            }

            self.sql.push_str(&h.column);
            self.sql.push(' ');
            self.sql.push_str(&h.operator);
            self.sql.push(' ');
            self.push_placeholder();
            h.value.bind_to(&mut self.arguments).unwrap();
        }
    }

    fn write_order_by(&mut self) {
        let stmt = self.statement;

        if stmt.order_by.is_empty() {
            return;
        }

        self.sql.push_str(" ORDER BY ");

        for (idx, o) in stmt.order_by.iter().enumerate() {
            if idx > 0 {
                self.sql.push_str(", ");
            }
            self.sql.push_str(&o.column);
            self.sql.push(' ');
            self.sql.push_str(&o.order.to_string());
        }
    }

    fn write_limit(&mut self) {
        if let Some(limit) = &self.statement.limit {
            limit.value.as_ref().bind_to(&mut self.arguments).unwrap();
            self.sql.push_str(" LIMIT ");
            self.push_placeholder();
        }
    }

    fn write_offset(&mut self) {
        if let Some(offset) = &self.statement.offset {
            offset.value.as_ref().bind_to(&mut self.arguments).unwrap();
            self.sql.push_str(" OFFSET ");
            self.push_placeholder();
        }
    }
}