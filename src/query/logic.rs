use std::marker::PhantomData;

use sqlx::{Encode, Type};

use crate::query;

#[derive(Clone, Debug, Default)]
pub enum Condition {
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
pub struct Where {
    pub condition: Option<Condition>,
    pub column: Option<String>,
    pub operator: Option<String>,
    pub group: Option<Box<Where>>
}

#[derive(Debug)]
pub struct WhereGroup<'q, DB: sqlx::Database> {
    pub queries: Vec<Where>,
    _marker: PhantomData<DB>,
    _life: PhantomData<&'q ()>
}

impl <'q, DB>WhereGroup<'q, DB>
where
    DB: sqlx::Database
{
    pub fn new() -> Self {
        return Self {
            queries: Vec::new(),
            _marker: PhantomData,
            _life: PhantomData
        }
    }

    pub fn r#where<T: 'q + Encode<'q, DB> + Type<DB>>(&mut self, _column: &str, _operator: &str, _val: T) -> &mut Self {
        todo!()
    }
}

#[derive(Clone, Debug)]
pub struct Order {
    pub column: String,
    pub order: query::Order,
}

#[derive(Clone, Debug, Default)]
pub enum JoinType {
    InnerJoin,
    #[default]
    LeftJoin,
    RightJoin,
    FullOuterJoin,
    CrossJoin
}

#[derive(Clone, Debug, Default)]
pub struct Join {
    pub table: String,
    pub column: String,
    pub operator: String,
    pub column_table: String, 
    pub join_type: JoinType
}

#[derive(Clone, Debug, Default)]
pub struct Having {
    pub column: String,
    pub operator: String,
    pub value: String,
    pub position: Option<Condition>
}

#[derive(Clone, Debug, Default)]
pub struct SqlQuery {
    pub table: String,
    pub select: Vec<String>,
    pub join: Vec<Join>,
    pub where_queries: Vec<Where>,
    pub group_by: Option<String>,
    pub having: Vec<Having>,
    pub order_by: Vec<Order>,
    pub limit: Option<u64>,
    pub page: Option<u64>, // TODO: must use `offset` or `page` must decide...
    pub columns: Vec<String>,
}

impl SqlQuery {
    pub fn new(table: &str) -> Self {
        return Self {
            table: table.to_string(),
            select: Vec::new(),
            join: Vec::new(),
            where_queries: Vec::new(),
            having: Vec::new(),
            group_by: None,
            order_by: Vec::new(),
            limit: None,
            page: None,
            columns: Vec::new(),
        }
    }
}