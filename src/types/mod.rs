use sqlx::{
    Database as SqlxDatabase,
    Arguments,
    error::BoxDynError
};

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