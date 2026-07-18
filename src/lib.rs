use std::{collections::HashMap, error::Error};

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
    fn bind_to<'q>(&self, args: &mut <DB as SqlxDatabase>::Arguments<'q>) -> std::result::Result<(), BoxDynError>;
}

impl<DB, T> Bindable<DB> for T
where
    DB: SqlxDatabase,
    T: for<'q> sqlx::Encode<'q, DB> + sqlx::Type<DB> + Clone + Send + 'static,
{
    #[inline]
    fn bind_to<'q>(&self, args: &mut <DB as SqlxDatabase>::Arguments<'q>) -> std::result::Result<(), BoxDynError> {
        return args.add(self.clone());
    }
}

#[derive(Clone, Copy)]
pub enum Connector {
    And,
    Or,
}

impl ToString for Connector {
    fn to_string(&self) -> String {
        return match self {
            Connector::And => String::from("AND"),
            Connector::Or  => String::from("OR"),
        }
    }
}

pub enum WhereClause<DB: SqlxDatabase> {
    Clause {
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
            WhereClause::Clause { connector, .. } => *connector,
            WhereClause::Group { connector, .. } => *connector,
        }
    }
}

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
        self.conditions.push(WhereClause::Clause {
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
        self.conditions.push(WhereClause::Clause {
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

// pub struct Connection<E: Executor> {
//     inner: Box<E>
// }

// pub fn 

#[allow(async_fn_in_trait)]
pub trait Executor {
    type DB: SqlxDatabase;

    fn to_sql<'q>(&self, statement: &Statement<Self::DB>) -> String;

    // async fn execute_as<'q, O>(&self, sql: String) -> Result<Vec<O>>;

    async fn insert<'q>(&self) -> Result<()>;

    // async fn update<'q>(&self) -> Result<()>;

    // async fn count<'q>(&self) -> Result<u64>;

    // async fn delete<'q>(&self) -> Result<()>;

    // async fn insert_as<'q, O>(&self) -> Result<O>;

    // async fn query_all<'q, O>(&self, sql: &str) -> Result<Vec<O>>;

    // async fn query_one<'q, O>(&self, sql: &str) -> Result<O>;

    async fn all<O>(&self, statement: &Statement<Self::DB>) -> Result<Vec<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as SqlxDatabase>::Row> + Send + Unpin;

    async fn first<O>(&self, statement: &Statement<Self::DB>) -> Result<O>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as SqlxDatabase>::Row> + Send + Unpin;

    async fn get<O>(&self, statement: &Statement<Self::DB>) -> Result<Vec<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as SqlxDatabase>::Row> + Send + Unpin;
        
    async fn paginate<O>(&self, statement: &Statement<Self::DB>) -> Result<Pagination<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as SqlxDatabase>::Row> + Send + Unpin;
}

#[derive(Clone, Debug)]
pub enum JoinType {
    Join,
    InnerJoin,
    LeftJoin,
    RightJoin,
    FullOuterJoin,
    CrossJoin
}

impl ToString for JoinType {
    fn to_string(&self) -> String {
        return match self {
            JoinType::Join => "JOIN".into(),
            JoinType::LeftJoin => "LEFT JOIN".into(),
            JoinType::RightJoin => "RIGHT JOIN".into(),
            JoinType::InnerJoin => "INNER JOIN".into(),
            JoinType::FullOuterJoin => "FULL OUTER JOIN".into(),
            JoinType::CrossJoin => "CROSS JOIN".into(),
        };
    }
}

#[derive(Clone, Debug)]
pub struct Join {
    pub table: String,
    pub column: String,
    pub operator: String,
    pub column_table: String, 
    pub join_type: JoinType
}

pub struct Having<DB: SqlxDatabase> {
    pub column: String,
    pub operator: String,
    pub value: Box<dyn Bindable<DB>>,
    pub connector: Connector,
}

#[derive(Clone, Debug)]
pub enum Order {
    ASC,
    DESC
}

impl ToString for Order {
    fn to_string(&self) -> String {
        return match self {
            Order::ASC  => "ASC".into(),
            Order::DESC => "DESC".into(),
        };
    }
}

#[derive(Clone, Debug)]
pub struct OrderValue {
    pub column: String,
    pub order: Order,
}

impl OrderValue {
    pub fn new(column: impl Into<String>, order: Order) -> Self {
        return Self {
            column: column.into(),
            order: order
        };
    }
}

pub struct Limit<DB: SqlxDatabase> {
    pub value: Box<dyn Bindable<DB>>,
}

pub struct Offset<DB: SqlxDatabase> {
    pub value: Box<dyn Bindable<DB>>,
}

pub struct Statement<DB: SqlxDatabase> {
    pub table: String,
    pub fields: Vec<String>,
    pub join: Vec<Join>,
    pub conditions: Vec<WhereClause<DB>>,
    pub group_by: Option<String>,
    pub having: Vec<Having<DB>>,
    pub order_by: Vec<OrderValue>,
    pub limit: Option<Limit<DB>>,
    pub offset: Option<Offset<DB>>,
    pub values: HashMap<String, Box<dyn Bindable<DB>>>
}

impl <DB: SqlxDatabase>Statement<DB> {
    pub fn new(table: impl Into<String>) -> Self {
        return Self {
            table: table.into(),
            fields: Vec::new(),
            join: Vec::new(),
            conditions: Vec::new(),
            group_by: None,
            having: Vec::new(),
            order_by: Vec::new(),
            limit: None,
            offset: None,
            values: HashMap::new(),
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

    pub fn select(&mut self, fields: Vec<&str>) -> &mut Self {
        self.statement.fields = fields
            .iter()
            .map(|f| f.to_string())
            .collect();
        
        return self;
    }

    fn join_push(
        &mut self,
        join_type: JoinType,
        table: impl Into<String>,
        column: impl Into<String>,
        operator: impl Into<String>,
        column_table: impl Into<String>) -> &mut Self {
        self.statement.join.push(Join {
            table: table.into(),
            column: column.into(),
            operator: operator.into(),
            column_table: column_table.into(),
            join_type: join_type
        });
        return self;
    }

    pub fn join(
        &mut self,
        table: impl Into<String>,
        column: impl Into<String>,
        operator: impl Into<String>,
        column_table: impl Into<String>) -> &mut Self {
        return self.join_push(JoinType::Join , table, column, operator, column_table);
    }

    pub fn join_right(
        &mut self,
        table: impl Into<String>,
        column: impl Into<String>,
        operator: impl Into<String>,
        column_table: impl Into<String>) -> &mut Self
    {
        return self.join_push(JoinType::RightJoin , table, column, operator, column_table);
    }

    pub fn join_left(
        &mut self,
        table: impl Into<String>,
        column: impl Into<String>,
        operator: impl Into<String>,
        column_table: impl Into<String>) -> &mut Self
    {
        return self.join_push(JoinType::LeftJoin , table, column, operator, column_table);
    }

    pub fn join_inner(
        &mut self,
        table: impl Into<String>,
        column: impl Into<String>,
        operator: impl Into<String>,
        column_table: impl Into<String>) -> &mut Self
    {
        return self.join_push(JoinType::InnerJoin , table, column, operator, column_table);
    }

    pub fn join_full_outer(
        &mut self,
        table: impl Into<String>,
        column: impl Into<String>,
        operator: impl Into<String>,
        column_table: impl Into<String>) -> &mut Self
    {
        return self.join_push(JoinType::FullOuterJoin , table, column, operator, column_table);
    }

    pub fn join_cross(
        &mut self,
        table: impl Into<String>,
        column: impl Into<String>,
        operator: impl Into<String>,
        column_table: impl Into<String>) -> &mut Self
    {
        return self.join_push(JoinType::CrossJoin , table, column, operator, column_table);
    }

    pub fn r#where<V>(&mut self, c: impl Into<String>, o: impl Into<String>, v: V) -> &mut Self 
    where
        V: Bindable<E::DB>,
    {
        self.statement.conditions.push(WhereClause::Clause {
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
        self.statement.conditions.push(WhereClause::Clause {
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

    pub fn group_by(&mut self, column: impl Into<String>) -> &mut Self {
        self.statement.group_by = Some(column.into());

        return self;
    }

    fn having_push<V>(
        &mut self,
        connector: Connector,
        column: impl Into<String>,
        operator: impl Into<String>, value: V) -> &mut Self
    where
        V: Bindable<E::DB>
    {
        self
            .statement
            .having
            .push(Having {
                column: column.into(),
                operator: operator.into(),
                value: Box::new(value),
                connector,
            });

        return self;
    }

    pub fn having<V>(&mut self, column: impl Into<String>, operator: impl Into<String>, value: V) -> &mut Self
    where
        V: Bindable<E::DB>
    {
        return self.having_push(Connector::And, column, operator, value);
    }

    pub fn and_having<V>(&mut self, column: impl Into<String>, operator: impl Into<String>, value: V) -> &mut Self
    where
        V: Bindable<E::DB>
    {
        return self.having_push(Connector::And, column, operator, value);
    }

    pub fn or_having<V>(&mut self, column: impl Into<String>, operator: impl Into<String>, value: V) -> &mut Self
    where
        V: Bindable<E::DB>
    {
        return self.having_push(Connector::Or, column, operator, value);
    }

    pub fn order_by(&mut self, column: impl Into<String>, order: Order) -> &mut Self {
        self
            .statement
            .order_by
            .push(OrderValue::new(column, order));
        
        return self;
    }

    pub fn limit<V>(&mut self, limit: V) -> &mut Self
    where
        V: Bindable<E::DB>
    {
        self.statement.limit = Some(Limit { value: Box::new(limit) });

        return self;
    }

    pub fn offset<V>(&mut self, offset: V) -> &mut Self 
    where
        V: Bindable<E::DB>
    {
        self.statement.offset = Some(Offset { value: Box::new(offset) });

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


