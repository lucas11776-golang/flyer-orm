use crate::{Bindable, Having, Join, Limit, Offset, OrderValue, Statement, WhereClause, types::ArgsAsRef};

pub mod mysql;
pub mod postgres;
pub mod sqlite;

pub trait Builder<'c, DB: sqlx::Database> {
    fn new(dry_run: bool) -> Self;

    fn push_placeholder(&mut self, prefix: &str, suffix: &str);

    fn to_sql(&mut self, statement: &'c Statement<DB>) -> String;

    fn insert(&mut self, statement: &'c Statement<DB>) -> (String, ArgsAsRef<'c, DB>);

    fn update(&mut self, statement: &'c Statement<DB>) -> (String, ArgsAsRef<'c, DB>);

    fn delete(&mut self, statement: &'c Statement<DB>) -> (String, ArgsAsRef<'c, DB>);

    fn query(&mut self, statement: &'c Statement<DB>) -> (String, ArgsAsRef<'c, DB>);

    fn select(&mut self, fields: &[String]) -> &mut Self;

    fn from(&mut self, table: &str) -> &mut Self;

    fn joins(&mut self, joins: &'c [Join]) -> &mut Self;

    fn conditions(&mut self, conditions: &'c [WhereClause<DB>], is_root: bool) -> &mut Self;

    fn group_by(&mut self, group_by: &'c Option<String>) -> &mut Self;

    fn having(&mut self, having: &'c [Having<DB>]) -> &mut Self;

    fn order_by(&mut self, order_by: &'c [OrderValue]) -> &mut Self;

    fn limit(&mut self, limit: &'c Option<Limit<DB>>) -> &mut Self;

    fn offset(&mut self, offset: &'c Option<Offset<DB>>) -> &mut Self;

    fn compile(&mut self) -> (String, ArgsAsRef<'c, DB>);

    fn bind_to(&mut self, value: &'c Box<dyn Bindable<DB>>);
}