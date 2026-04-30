use std::{marker::PhantomData, str};

use anyhow::{Result};
use sqlx::{Arguments, Encode, types::Type};

use crate::{
    executor::Executor,
    query::Statement,
    types::{Condition, Where}
};

pub struct Update<'q, E: Executor> {
    db: &'q E,
    statement: &'q mut Statement<'q, E::T>,
    _marker: PhantomData<E>
}

// TODO: Need to have two arguments of insert and where value bindings.
// Currently -> Insert and Where in order
impl <'q, E>Update<'q, E>
where
    E: Executor
{
    pub(crate) fn new(db: &'q E, statement: &'q mut Statement<'q, E::T>) -> Self {
        return Self {
            db: db,
            statement: statement,
            _marker: PhantomData,
        }
    }

    pub fn r#where<T: 'q + Encode<'q, E::T> + Type<E::T>>(&mut self, column: &str, operator: &str, value: T) -> &mut Self {
        if self.statement.query.where_queries.len() != 0 {
            return self.and_where(column, operator, value);
        }

        self.statement.query.where_queries.push(Where {
            column: Some(column.to_string()),
            operator: Some(operator.to_string()),
            condition: None,
            group: None
        });

        self.statement.arguments.add(value).unwrap();
        
        return self;
    }

    pub fn and_where<T: 'q + Encode<'q, E::T> + Type<E::T>>(&mut self, column: &str, operator: &str, value: T) -> &mut Self {
        if self.statement.query.where_queries.len() == 0 {
            return self.r#where(column, operator, value);
        }

        self.statement.query.where_queries.push(Where {
            column: Some(column.to_string()),
            operator: Some(operator.to_string()),
            condition: Some(Condition::AND),
            group: None
        });

        self.statement.arguments.add(value).unwrap();
        
        return self;
    }

    pub fn or_where<T: 'q + Encode<'q, E::T> + Type<E::T>>(&mut self, column: &str, operator: &str, value: T) -> &mut Self {
        if self.statement.query.where_queries.len() == 0 {
            return self.r#where(column, operator, value);
        }

        self.statement.query.where_queries.push(Where {
            column: Some(column.to_string()),
            operator: Some(operator.to_string()),
            condition: Some(Condition::OR),
            group: None
        });

        self.statement.arguments.add(value).unwrap();
        
        return self;
    }

    pub fn bind<T: 'q + Encode<'q, E::T> + Type<E::T>>(&'q mut self, value: T) -> &'q mut Self {
        self.statement.arguments.add(value).unwrap();
        return self;
    }

    // pub fn where_group(&mut self, callback: fn(group: WhereQueryGroup<'q, E::T>) -> WhereQueryGroup<'q, E::T>) -> &mut Self {        
    //     return self;
    // }

    // pub fn and_where_group(&mut self, callback: fn(group: WhereQueryGroup<'q, E::T>) -> WhereQueryGroup<'q, E::T>) -> &mut Self {        
    //     return self;
    // }

    // pub fn or_where_group(&mut self, callback: fn(group: WhereQueryGroup<'q, E::T>) -> WhereQueryGroup<'q, E::T>) -> &mut Self {        
    //     return self;
    // }

    pub async fn execute(&'q mut self) -> Result<()> {
        return self.db.update(self.statement).await;
    }
}
