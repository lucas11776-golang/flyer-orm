use std::any::Any;

use sqlx::{Arguments, error::BoxDynError};

use crate::Result;

pub trait Bindable<DB: sqlx::Database>: Send + Sync + 'static {
    fn bind_to<'q>(&self, args: &mut <DB as sqlx::Database>::Arguments<'q>) -> Result<(), BoxDynError>;
    fn as_any(&self) -> &dyn Any;
}

impl<DB, T> Bindable<DB> for T
where
    DB: sqlx::Database,
    T: for<'q> sqlx::Encode<'q, DB> + sqlx::Type<DB> + Clone + Send + Sync + 'static,
{
    #[inline]
    fn bind_to<'q>(&self, args: &mut <DB as sqlx::Database>::Arguments<'q>) -> Result<(), BoxDynError> {
        return args.add(self.clone());
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<DB: sqlx::Database> dyn Bindable<DB> {
    pub fn parse<T: 'static + Clone>(&self) -> Option<T> {
        self
            .as_any()
            .downcast_ref::<T>()
            .cloned()
    }
}

pub trait QueryResult {
    fn rows_affected(&self) -> u64;
    fn last_inserted(&self) -> u64;
}

#[derive(Clone, Copy)]
pub enum Connector {
    And,
    Or,
}

impl ToString for Connector {
    fn to_string(&self) -> String {
        return match self {
            Connector::And => "AND".into(),
            Connector::Or  => "OR".into(),
        }
    }
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
            JoinType::Join          => "JOIN".into(),
            JoinType::LeftJoin      => "LEFT JOIN".into(),
            JoinType::RightJoin     => "RIGHT JOIN".into(),
            JoinType::InnerJoin     => "INNER JOIN".into(),
            JoinType::FullOuterJoin => "FULL OUTER JOIN".into(),
            JoinType::CrossJoin     => "CROSS JOIN".into(),
        };
    }
}

pub enum WhereClause<DB: sqlx::Database> {
    Clause {
        column: String,
        operator: String,
        value: Box<dyn Bindable<DB>>,
        connector: Connector,
    },
    NullCheck {
        column: String,
        is_null: bool,
        connector: Connector,
    },
    In {
        column: String,
        negated: bool,
        values: Vec<Box<dyn Bindable<DB>>>,
        connector: Connector,
    },
    Between {
        column: String,
        negated: bool,
        low: Box<dyn Bindable<DB>>,
        high: Box<dyn Bindable<DB>>,
        connector: Connector,
    },
    Group {
        conditions: Vec<WhereClause<DB>>,
        connector: Connector,
    },
}

impl<DB: sqlx::Database> WhereClause<DB> {
    pub fn connector(&self) -> Connector {
        match self {
            WhereClause::Clause { connector, .. }    => *connector,
            WhereClause::Group { connector, .. }     => *connector,
            WhereClause::NullCheck { connector, .. } => *connector,
            WhereClause::In { connector, .. }        => *connector,
            WhereClause::Between { connector, .. }   => *connector,
        }
    }
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
