use std::marker::PhantomData;

use sqlx::{Encode, Type};

use crate::types::Where;

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