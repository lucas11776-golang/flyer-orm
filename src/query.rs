use serde::Serialize;
use sqlx::Transaction as SqlxTransaction;


// TODO: add value here maybe...
#[derive(Clone, Debug)]
pub(crate) struct WhereQuery {
    pub column: String,
    pub operator: String,
    pub position: Option<WhereQueryPosition>,
}

#[derive(Clone, Debug)]
pub(crate) enum WhereQueryPosition {
    AND,
    OR
}

#[derive(Clone, Debug)]
pub(crate) struct OrderQuery {
    pub column: String,
    pub order: Order
}

#[derive(Clone, Debug)]
pub(crate) struct JoinQuery {
    pub table: String,
    pub column: String,
    pub operator: String,
    pub column_table: String, 
}

#[derive(Clone, Debug)]
pub struct Statement<'q, DB: sqlx::Database> {
    pub(crate) url: String,
    pub(crate) table: String,
    pub(crate) select: Vec<String>,
    pub(crate) where_queries: Vec<WhereQuery>,
    pub(crate) join: Vec<JoinQuery>,
    pub(crate) order_by: Vec<OrderQuery>,
    pub(crate) limit: Option<u64>,
    pub(crate) page: Option<u64>, // TODO: must use `offset` or `page` must decide...
    pub(crate) arguments: DB::Arguments<'q>, 
}

impl <'q, DB>Statement<'q, DB>
where
    DB: sqlx::Database
{
    pub(crate) fn new(url: &str) -> Self {
        return Self {
            url: url.to_owned(),
            table: String::new(),
            select: Vec::new(),
            where_queries: Vec::new(),
            join: Vec::new(),
            order_by: Vec::new(),
            limit: None,
            page: None,
            arguments: Default::default(),
        }
    }
}

pub struct Transaction<'t, T: sqlx::Database> {
    transaction: SqlxTransaction<'t, T>
}

impl <'t, T: sqlx::Database>Transaction<'t, T> {
    fn new(transaction: SqlxTransaction<'t, T>) -> Self {
        return Self {
            transaction: transaction
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct Pagination<Entity> {
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
    pub items: Vec<Entity>
}

#[derive(Clone, Debug)]
pub enum Order {
    ASC,
    DESC
}

#[derive(Debug, sqlx::FromRow)]
pub(crate) struct Total {
    pub total: u64
}
    
