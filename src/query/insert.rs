use std::marker::PhantomData;

use anyhow::Result;
use sqlx::{Arguments, Encode, types::Type};

use crate::{executor::Executor, query::Statement};

pub struct Insert<'q, E: Executor> {
    db: &'q E,
    statement: &'q mut Statement<E::T>,
    _marker: PhantomData<E>
}

impl <'q, E>Insert<'q, E>
where
    E: Executor
{
    pub(crate) fn new(db: &'q E, statement: &'q mut Statement<E::T>) -> Self {
        return Self {
            db: db,
            statement: statement,
            _marker: PhantomData,
        }
    }

    pub fn bind<T: 'q + Encode<'q, E::T> + Type<E::T>>(&'q mut self, value: T) -> &'q mut Self {
        self.statement.arguments.add(value).unwrap();

        return self;
    }

    pub async fn execute(&'q mut self) -> Result<()> {
        return self.db.insert(self.statement).await;
    }
}
