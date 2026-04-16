use std::marker::PhantomData;

use anyhow::Result;
use sqlx::{Arguments, Encode, FromRow, types::Type};

use crate::{executor::Executor, query::Statement};

pub struct InsertAs<'q, E: Executor, O> {
    db: &'q E,
    statement: &'q mut Statement<'q, E::T>,
    _marker: PhantomData<E>,
    _type: PhantomData<O>
}

impl <'q, E, O>InsertAs<'q, E, O>
where
    E: Executor,
    O: for<'r> FromRow<'r, <E::T as sqlx::Database>::Row> + Send + Unpin + Sized
{
    pub(crate) fn new(db: &'q E, statement: &'q mut Statement<'q, E::T>) -> Self {
        return Self {
            db: db,
            statement: statement,
            _marker: PhantomData,
            _type: PhantomData
        }
    }

    pub fn bind<T: 'q + Encode<'q, E::T> + Type<E::T>>(&'q mut self, value: T) -> &'q mut Self {
        self.statement.arguments.add(value).unwrap();

        return self;
    }

    pub async fn execute(&'q mut self) -> Result<O> {
        return self.db.insert_as::<O>(self.statement).await;
    }
}
