use std::error::Error;

use sqlx::{Database as SqlxDatabase};

pub use derive::Entity;
pub use database::postgres::Postgres;
pub use database::mysql::MySQL;
pub use database::sqlite::SQLite;

use crate::{
    query::{Having, Join, Limit, Offset, OrderValue, Pagination, Statement, WhereGroup, execute_as::ExecuteAs},
    types::{Bindable, Connector, JoinType, Order}
};

pub mod database;
pub mod types;
pub mod query;

pub trait Entity {}

pub type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

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

#[allow(async_fn_in_trait)]
pub trait Executor {
    type DB: SqlxDatabase;

    fn to_sql<'q>(&self, statement: &Statement<Self::DB>) -> String;

    async fn fetch_one<'c, O>(&self, sql: String, arguments: <Self::DB as sqlx::Database>::Arguments<'c>) -> Result<O>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as SqlxDatabase>::Row> + Send + Unpin;

    async fn fetch_all<'c, O>(&self, sql: String, arguments: <Self::DB as sqlx::Database>::Arguments<'c>) -> Result<Vec<O>>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as SqlxDatabase>::Row> + Send + Unpin;

    async fn insert<'q>(&self, statement: &Statement<Self::DB>) -> Result<()>;

    async fn update<'q>(&self, statement: &Statement<Self::DB>) -> Result<()>;

    async fn count<'q>(&self, statement: &Statement<Self::DB>) -> Result<u64>;

    async fn delete<'q>(&self, statement: &Statement<Self::DB>) -> Result<()>;

    async fn insert_as<'q, O>(&self, statement: &Statement<Self::DB>) -> Result<O>
    where
        O: Entity + for<'r> sqlx::FromRow<'r, <Self::DB as SqlxDatabase>::Row> + Send + Unpin;

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

    // pub async fn get<'c, O>(&mut self) -> Result<Vec<O>>
    // where
    //     O: Entity + for<'r> sqlx::FromRow<'r, <E::DB as SqlxDatabase>::Row> + Send + Unpin, 
    //     for<'a> <E::DB as SqlxDatabase>::Arguments<'a>: IntoArguments<'a, E::DB>,
    // {
    //     return self
    //         .executor
    //         .get(&self.statement)
    //         .await;
    // }


    pub fn execute_as(&self, sql: impl Into<String>) -> ExecuteAs<'_, E> {
        return ExecuteAs::new(self.executor, sql);
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
        // for<'a> <E::DB as SqlxDatabase>::Arguments<'a>: IntoArguments<'a, E::DB>,
    {
        return self
            .executor
            .get(&self.statement)
            .await;
    }
}