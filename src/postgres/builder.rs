use std::{fmt::Write, mem};

use crate::{
    Bindable,
    Having,
    Join,
    Limit,
    Offset,
    OrderValue,
    Statement,
    WhereClause
};

pub struct QueryBuilder<'c, DB: sqlx::Database> {
    arguments: <DB as sqlx::Database>::Arguments<'c>,
    params_index: i32,
    sql: String,
    dry_run: bool,
}

impl<'c, DB: sqlx::Database> QueryBuilder<'c, DB> {
    pub fn new(dry_run: bool) -> Self {
        Self {
            arguments: Default::default(),
            params_index: 0,
            sql: String::new(),
            dry_run,
        }
    }

    #[inline]
    fn push_placeholder(&mut self) {
        self.params_index += 1;
        let _ = write!(self.sql, "${}", self.params_index);
    }

    pub fn query(&mut self, statement: &Statement<DB>) -> (String, <DB as sqlx::Database>::Arguments<'c>) {
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

    // Optimization: Changed &Vec<String> to &[String]
    pub fn select(&mut self, fields: &[String]) -> &mut Self {
        self.sql.push_str("SELECT ");

        if fields.is_empty() {
            self.sql.push('*');
            return self;
        }

        // Optimization: Avoids the heap allocation caused by fields.join(", ")
        for (idx, field) in fields.iter().enumerate() {
            if idx > 0 {
                self.sql.push_str(", ");
            }
            self.sql.push_str(field);
        }
        
        return self;
    }

    pub fn from(&mut self, table: &str) -> &mut Self {
        self.sql.push_str(" FROM ");
        self.sql.push_str(table);

        return self;
    }

    pub fn joins(&mut self, joins: &[Join]) -> &mut Self {
        for join in joins {
            let _ = write!(
                self.sql,
                " {} {} ON {} {} {}",
                join.join_type.to_string(), join.table, join.column, join.operator, join.column_table
            );
        }

        return self;
    }

    pub fn conditions(&mut self, conditions: &[WhereClause<DB>], is_root: bool) -> &mut Self {
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
                WhereClause::Clause { column, operator, value, .. } => {
                    let _ = write!(self.sql, "{} {} ", column, operator);
                    self.push_placeholder();
                    self.bind_to(value);
                }
                WhereClause::Group { conditions: sub_conds, .. } => {
                    self.sql.push('(');
                    self.conditions(sub_conds, false);
                    self.sql.push(')');
                }
            }
        }

        return self;
    }

    pub fn group_by(&mut self, group_by: &Option<String>) -> &mut Self {
        if let Some(column) = group_by {
            self.sql.push_str(" GROUP BY ");
            self.sql.push_str(column);
        }

        return self;
    }

    pub fn having(&mut self, having: &[Having<DB>]) -> &mut Self {
        if having.is_empty() {
            return self;
        }

        self.sql.push_str(" HAVING ");

        for (idx, h) in having.iter().enumerate() {
            if idx > 0 {
                let _ = write!(self.sql, " {} ", h.connector.to_string());
            }

            let _ = write!(self.sql, "{} {} ", h.column, h.operator);

            self.push_placeholder();
            self.bind_to(&h.value);
        }

        return self;
    }
    
    pub fn order_by(&mut self, order_by: &[OrderValue]) -> &mut Self {
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

        return self;
    }

    pub fn limit(&mut self, limit: &Option<Limit<DB>>) -> &mut Self {
        if let Some(limit) = limit {
            self.bind_to(&limit.value);
            self.sql.push_str(" LIMIT ");
            self.push_placeholder();
        }

        return self;
    }

    pub fn offset(&mut self, offset: &Option<Offset<DB>>) -> &mut Self {
        if let Some(offset) = offset {
            self.bind_to(&offset.value);
            self.sql.push_str(" OFFSET ");
            self.push_placeholder();
        }

        return self;
    }

    pub fn compile(&mut self) -> (String, <DB as sqlx::Database>::Arguments<'c>) {
        (mem::take(&mut self.sql), mem::take(&mut self.arguments))
    }

    fn bind_to(&mut self, value: &Box<dyn Bindable<DB>>) {
        if !self.dry_run {
            value
                .as_ref()
                .bind_to(&mut self.arguments)
                .unwrap();
        }
    }
}