use std::marker::PhantomData;

use sqlx::{Encode, Type};

use crate::query;

#[derive(Clone, Debug, Default)]
pub(crate) enum Condition {
    #[default]
    AND,
    OR
}

impl ToString for Condition {
    fn to_string(&self) -> String {
        return match self {
            Condition::AND => String::from("AND"),
            Condition::OR => String::from("OR"),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Where {
    pub condition: Option<Condition>,
    pub column: Option<String>,
    pub operator: Option<String>,
    pub group: Option<Box<Where>>
}

#[derive(Debug)]
pub struct WhereGroup<'q, DB: sqlx::Database> {
    pub(crate) _queries: Vec<Where>,
    _marker: PhantomData<DB>,
    _life: PhantomData<&'q ()>
}

impl <'q, DB>WhereGroup<'q, DB>
where
    DB: sqlx::Database
{
    pub fn new() -> Self {
        return Self {
            _queries: Vec::new(),
            _marker: PhantomData,
            _life: PhantomData
        }
    }

    pub fn r#where<T: 'q + Encode<'q, DB> + Type<DB>>(&mut self, _column: &str, _operator: &str, _val: T) -> &mut Self {
        todo!()
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Order {
    pub column: String,
    pub order: query::Order,
}

#[derive(Clone, Debug)]
pub(crate) enum JoinType {
    InnerJoin,
    LeftJoin,
    RightJoin,
    FullOuterJoin,
    CrossJoin
}

#[derive(Clone, Debug)]
pub(crate) struct Join {
    pub table: String,
    pub column: String,
    pub operator: String,
    pub column_table: String, 
    pub join_type: JoinType
}

#[derive(Clone, Debug, Default)]
pub(crate) struct Having {
    pub column: String,
    pub operator: String,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct SqlQuery {
    pub table: String,
    pub select: Vec<String>,
    pub join: Vec<Join>,
    pub where_queries: Vec<Where>,
    pub group_by: Option<String>,
    pub having: Option<Having>,
    pub order_by: Option<Vec<Order>>,
    pub limit: Option<i64>,
    pub page: Option<i64>, // TODO: must use `offset` or `page` must decide...
    pub columns: Vec<String>,
}

impl SqlQuery {
    pub fn new(table: &str) -> Self {
        return Self {
            table: table.to_string(),
            select: Vec::new(),
            join: Vec::new(),
            where_queries: Vec::new(),
            having: None,
            group_by: None,
            order_by: None,
            limit: None,
            page: None,
            columns: Vec::new(),
        }
    }
}