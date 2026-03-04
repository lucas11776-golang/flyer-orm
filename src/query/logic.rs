use std::marker::PhantomData;

use sqlx::{Encode, Type};

use crate::query::Order;

#[derive(Clone, Debug, Default)]
pub enum Condition {
    #[default]
    AND,
    OR
}

#[derive(Clone, Debug)]
pub struct Where {
    pub column: Option<String>,
    pub operator: Option<String>,
    pub position: Option<Condition>,
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
pub struct OrderQuery {
    pub column: String,
    pub order: Order
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