use crate::query::QueryResult;

pub struct SQLiteQueryResult {
    pub(crate) affected: u64,
    pub(crate) id: u64,
}

impl QueryResult for SQLiteQueryResult {
    fn rows_affected(&self) -> u64 {
        return self.affected;
    }

    fn last_inserted(&self) -> u64 {
        return self.id;
    }
}