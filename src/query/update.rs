use std::{marker::PhantomData, mem::take, str};

use anyhow::{Result};
use sqlx::{Arguments, Encode, types::Type};

use crate::{
    executor::Executor,
    query::Statement,
    types::{Condition, Where}
};

pub struct Update<'q, E: Executor> {
    db: &'q E,
    statement: &'q mut Statement<E::T>,
    insert_arguments: <E::T as sqlx::Database>::Arguments,
    where_arguments: <E::T as sqlx::Database>::Arguments,
    _marker: PhantomData<E>
}

impl <'q, E>Update<'q, E>
where
    E: Executor
{
    pub(crate) fn new(db: &'q E, statement: &'q mut Statement<E::T>) -> Self {
        return Self {
            db: db,
            statement: statement,
            insert_arguments: Default::default(),
            where_arguments: Default::default(),
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

        self.where_arguments.add(value).unwrap();
        
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

        self.where_arguments.add(value).unwrap();
        
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

        self.where_arguments.add(value).unwrap();
        
        return self;
    }

    pub fn bind<T: 'q + Encode<'q, E::T> + Type<E::T>>(&'q mut self, value: T) -> &'q mut Self {
        self.insert_arguments.add(value).unwrap();
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

    pub async fn execute(&mut self) -> Result<()> {
        self.statement.arguments = Default::default();

        self.statement.arguments.merge(take(&mut self.insert_arguments));
        self.statement.arguments.merge(take(&mut self.where_arguments));
        
        return self.db.update(self.statement).await;
    }
}