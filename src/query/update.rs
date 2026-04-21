
use std::{marker::PhantomData, str};

use anyhow::{Result};
use sqlx::{Arguments, Encode, IntoArguments, types::Type};

use crate::{
    executor::Executor,
    query::Statement,
    types::{Condition, Where}
};



pub struct Update<'q, E: Executor, T: 'q + Encode<'q, E::T> + Type<E::T>> {
    db: &'q E,
    statement: &'q mut Statement<'q, E::T>,
    insert_arguments: Vec<T>,
    where_arguments: Vec<T>,
    _marker: PhantomData<E>
}

impl <'q, E, T: 'q + Encode<'q, E::T> + Type<E::T>>Update<'q, E, T>
where
    E: Executor
{
    pub(crate) fn new(db: &'q E, statement: &'q mut Statement<'q, E::T>) -> Self {
        return Self {
            db: db,
            statement: statement,
            insert_arguments: Default::default(),
            where_arguments: Default::default(),
            _marker: PhantomData,
        }
    }

    pub fn r#where<W: 'q + Encode<'q, E::T> + Type<E::T>>(&mut self, column: &str, operator: &str, value: W) -> &mut Self {
        if self.statement.query.where_queries.len() != 0 {
            return self.and_where(column, operator, value);
        }

        self.statement.query.where_queries.push(Where {
            column: Some(column.to_string()),
            operator: Some(operator.to_string()),
            condition: None,
            group: None
        });

        self.where_arguments.push(unsafe { std::mem::transmute_copy(&10) });
        
        return self;
    }

    pub fn and_where<W: 'q + Encode<'q, E::T> + Type<E::T>>(&mut self, column: &str, operator: &str, value: W) -> &mut Self {
        if self.statement.query.where_queries.len() == 0 {
            return self.r#where(column, operator, value);
        }

        self.statement.query.where_queries.push(Where {
            column: Some(column.to_string()),
            operator: Some(operator.to_string()),
            condition: Some(Condition::AND),
            group: None
        });

        self.where_arguments.push(unsafe { std::mem::transmute_copy(&value) });
        
        return self;
    }

    pub fn or_where<W: 'q + Encode<'q, E::T> + Type<E::T>>(&mut self, column: &str, operator: &str, value: W) -> &mut Self {
        if self.statement.query.where_queries.len() == 0 {
            return self.r#where(column, operator, value);
        }

        self.statement.query.where_queries.push(Where {
            column: Some(column.to_string()),
            operator: Some(operator.to_string()),
            condition: Some(Condition::OR),
            group: None
        });

        self.where_arguments.push(unsafe { std::mem::transmute_copy(&value) });

        return self;
    }

    pub fn bind<W: 'q + Encode<'q, E::T> + Type<E::T>>(&'q mut self, value: W) -> &'q mut Self {
        self.insert_arguments.push(unsafe { std::mem::transmute_copy(&value) });
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
        self.statement.arguments = Default::default();

        for v in self.insert_arguments.iter() {
            self.statement.arguments.add(v).unwrap();
        }

        for v in self.where_arguments.iter() {
            self.statement.arguments.add(v).unwrap();
        }
        
        return self.db.update(self.statement).await;
    }
}
