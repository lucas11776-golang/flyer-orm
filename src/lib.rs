use std::error::Error;

use serde::Serialize;
use sqlx::{
    Database as SqlxDatabase,
    IntoArguments,
    Arguments,
    error::BoxDynError
};

pub use derive::Entity;

pub mod mysql;
pub mod postgres;
pub mod sqlite;
pub mod types;

pub trait Entity {}

pub type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

pub trait Bindable<DB: SqlxDatabase>: Send + 'static {
    fn bind_to<'q>(self: Box<Self>, args: &mut <DB as SqlxDatabase>::Arguments<'q>) -> std::result::Result<(), BoxDynError>;
}

impl<DB, T> Bindable<DB> for T
where
    DB: SqlxDatabase,
    T: for<'q> sqlx::Encode<'q, DB> + sqlx::Type<DB> + Send + 'static,
{
    #[inline]
    fn bind_to<'q>(self: Box<Self>, args: &mut <DB as SqlxDatabase>::Arguments<'q>) -> std::result::Result<(), BoxDynError> {
        return args.add(*self);
    }
}

#[derive(Clone, Copy)]
pub enum Connector {
    And,
    Or,
}

pub enum WhereClause<DB: SqlxDatabase> {
    Simple {
        connector: Connector,
        column: String,
        operator: String,
        value: Box<dyn Bindable<DB>>,
    },
    Group {
        connector: Connector,
        conditions: Vec<WhereClause<DB>>,
    },
}

impl<DB: SqlxDatabase> WhereClause<DB> {
    fn connector(&self) -> Connector {
        match self {
            WhereClause::Simple { connector, .. } => *connector,
            WhereClause::Group { connector, .. } => *connector,
        }
    }
}

// Clauses

pub struct WhereGroup<DB: SqlxDatabase> {
    pub conditions: Vec<WhereClause<DB>>,
}

impl<DB: SqlxDatabase> WhereGroup<DB> {
    pub fn new() -> Self {
        return Self {
            conditions: Vec::new()
        };
    }

    pub fn r#where<V>(&mut self, c: impl Into<String>, o: impl Into<String>, v: V) -> &mut Self 
    where
        V: Bindable<DB>,
    {
        self.conditions.push(WhereClause::Simple {
            connector: Connector::And,
            column: c.into(),
            operator: o.into(),
            value: Box::new(v),
        });
        self
    }

    pub fn and_where<V>(&mut self, c: impl Into<String>, o: impl Into<String>, v: V) -> &mut Self 
    where
        V: Bindable<DB>,
    {
        self.r#where(c, o, v)
    }

    pub fn or_where<V>(&mut self, c: impl Into<String>, o: impl Into<String>, v: V) -> &mut Self 
    where
        V: Bindable<DB>,
    {
        self.conditions.push(WhereClause::Simple {
            connector: Connector::Or,
            column: c.into(),
            operator: o.into(),
            value: Box::new(v),
        });
        self
    }
}



#[derive(Serialize, Clone, Debug, Default)]
pub struct Pagination<Entity> {
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
    pub items: Vec<Entity>
}

pub struct Connection<E: Executor> {
    inner: Box<E>
}

// pub fn 

#[allow(async_fn_in_trait)]
pub trait Executor {
    type DB: SqlxDatabase;
    // async fn new(url: &str) -> Self where Self: Sized;

    // fn to_sql<'q>(&self) -> String;

    // async fn execute_as<'q, O>(&self, sql: String) -> Result<Vec<O>>;

    // async fn insert<'q>(&self) -> Result<()>;

    // async fn update<'q>(&self) -> Result<()>;

    // async fn count<'q>(&self) -> Result<u64>;

    // async fn delete<'q>(&self) -> Result<()>;

    // async fn insert_as<'q, O>(&self) -> Result<O>;

    // async fn query_all<'q, O>(&self, sql: &str) -> Result<Vec<O>>;

    // async fn query_one<'q, O>(&self, sql: &str) -> Result<O>;

    // async fn all<'q, O>(&self) -> Result<Vec<O>>;

    async fn first<'e, O: Entity>(&self, statement: &Statement<Self::DB>) -> Result<O>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as SqlxDatabase>::Row> + Send + Unpin;

    async fn get<'e, O: Entity>(&self, statement: &Statement<Self::DB>) -> Result<Vec<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as SqlxDatabase>::Row> + Send + Unpin;
}

#[derive(Clone, Debug)]
pub(crate) enum JoinType {
    InnerJoin,
    LeftJoin,
    RightJoin,
    FullOuterJoin,
    CrossJoin
}

#[derive(Clone, Debug)]
pub(crate) struct Join {
    pub table: String,
    pub column: String,
    pub operator: String,
    pub column_table: String, 
    pub join_type: JoinType
}

pub(crate) struct Having<DB: SqlxDatabase> {
    pub column: String,
    pub operator: String,
    pub value: Box<dyn Bindable<DB>>
}

#[derive(Clone, Debug)]
pub enum OrderType {
    ASC,
    DESC
}

#[derive(Clone, Debug)]
pub(crate) struct Order {
    pub column: String,
    pub order: OrderType,
}

pub struct Statement<DB: SqlxDatabase> {
    pub table: String,
    pub fields: Vec<String>,
    pub join: Vec<Join>,
    pub conditions: Vec<WhereClause<DB>>,
    pub group_by: Option<String>,
    pub having: Option<Having<DB>>,
    pub order_by: Option<Vec<Order>>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl <DB: SqlxDatabase>Statement<DB> {
    pub fn new(table: impl Into<String>) -> Self {
        return Self {
            table: table.into(),
            fields: Vec::new(),
            join: Vec::new(),
            conditions: Vec::new(),
            group_by: None,
            having: None,
            order_by: None,
            limit: None,
            offset: None,
        };
    }
}

pub struct Query<'q, E: Executor> {
    executor: &'q E,
    statement: Statement<E::DB>,
}

impl <'q, E: Executor>Query<'q, E> {
    pub fn new(executor: &'q E, table: impl Into<String>) -> Self {
        return Self {
            executor: executor,
            statement: Statement::new(table),
        };
    }

    pub fn select(&mut self, fields: &[&str]) -> &mut Self {
        self.statement.fields = fields
            .iter()
            .map(|f| f.to_string())
            .collect();
        
        return self;
    }

    pub fn r#where<V>(&mut self, c: impl Into<String>, o: impl Into<String>, v: V) -> &mut Self 
    where
        V: Bindable<E::DB>,
    {
        self.statement.conditions.push(WhereClause::Simple {
            connector: Connector::And,
            column: c.into(),
            operator: o.into(),
            value: Box::new(v),
        });

        return self;
    }

    pub fn and_where<V>(&mut self, c: impl Into<String>, o: impl Into<String>, v: V) -> &mut Self 
    where
        V: Bindable<E::DB>,
    {
        return self.r#where(c, o, v);
    }

    pub fn or_where<V>(&mut self, c: impl Into<String>, o: impl Into<String>, v: V) -> &mut Self 
    where
        V: Bindable<E::DB>,
    {
        self.statement.conditions.push(WhereClause::Simple {
            connector: Connector::Or,
            column: c.into(),
            operator: o.into(),
            value: Box::new(v),
        });

        return self;
    }

    pub fn where_group<F>(&mut self, callback: F) -> &mut Self 
    where
        F: FnOnce(&mut WhereGroup<E::DB>),
    {
        let mut group = WhereGroup::new();

        callback(&mut group);
        
        self.statement.conditions.push(WhereClause::Group {
            connector: Connector::And,
            conditions: group.conditions,
        });

        return self;
    }

    pub async fn get<'c, O>(&mut self) -> Result<Vec<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <E::DB as SqlxDatabase>::Row> + Send + Unpin, 
        for<'a> <E::DB as SqlxDatabase>::Arguments<'a>: IntoArguments<'a, E::DB>,
    {
        return self
            .executor
            .get(&self.statement)
            .await;
    }
}