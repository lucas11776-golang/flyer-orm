use anyhow::Result;
use serde::Serialize;
use sqlx::Transaction as SqlxTransaction;


// TODO: add value here maybe...
#[derive(Clone, Debug)]
pub struct WhereQuery {
    pub column: String,
    pub operator: String,
    pub position: Option<QueryPosition>,
}

#[derive(Clone, Debug)]
pub enum QueryPosition {
    AND,
    OR
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
pub struct JoinQuery {
    pub table: String,
    pub column: String,
    pub operator: String,
    pub column_table: String, 
    pub join_type: JoinType
}

#[derive(Clone, Debug)]
pub struct Statement<'q, DB: sqlx::Database> {
    // pub url: String,
    pub query: QueryStatement,
    pub arguments: DB::Arguments<'q>, 
}

#[derive(Clone, Debug, Default)]
pub struct HavingQuery {
    pub column: String,
    pub operator: String,
    pub value: String,
    pub position: Option<QueryPosition>
}

#[derive(Clone, Debug, Default)]
pub struct QueryStatement {
    pub table: String,
    pub select: Vec<String>,
    pub join: Vec<JoinQuery>,
    pub where_queries: Vec<WhereQuery>,
    pub group_by: Option<String>,
    pub having: Vec<HavingQuery>,
    pub order_by: Vec<OrderQuery>,
    pub limit: Option<u64>,
    pub page: Option<u64>, // TODO: must use `offset` or `page` must decide...
}

impl QueryStatement {
    pub fn new() -> Self {
        return Self {
            table: String::new(),
            select: Vec::new(),
            join: Vec::new(),
            where_queries: Vec::new(),
            having: Vec::new(),
            group_by: None,
            order_by: Vec::new(),
            limit: None,
            page: None,
        }
    }
}

impl <'q, DB>Statement<'q, DB>
where
    DB: sqlx::Database
{
    pub(crate) fn new() -> Self {
        return Self {
            // url: url.to_owned(),
            query: QueryStatement::new(),
            arguments: Default::default(),
        }
    }
}

// TODO: fix connection must be some connection. -> query()
#[derive(Debug)]
pub struct Transaction<'t, T: sqlx::Database> {
    transaction: SqlxTransaction<'t, T>
}

impl <'t, T: sqlx::Database>Transaction<'t, T> {
    pub(crate) fn new(transaction: SqlxTransaction<'t, T>) -> Self {
        return Self {
            transaction: transaction
        }
    }

    pub async fn commit(self) -> Result<()> {
        return self.transaction.commit().await.map_err(|e| e.into());
    }

    pub async fn rollback(self) -> Result<()> {
        return self.transaction.rollback().await.map_err(|e| e.into());
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
    
