use flyer_orm::{Database, Executor, MySQL, Postgres, SQLite, Result};

/// Helper function for simple database usage you may as pass database name as
/// parameter if you have multiple database connections such as order, payments etc databases.
/// 
/// ```rust,no_run
/// pub db<'a>(connection: &str) -> Database<'a, Postgres> {
///     Database::<Postgres>::get(connection)
/// } 
/// ```
pub fn db<'a>() -> Database<'a, Postgres> {
    Database::<Postgres>::connection("SQLITE")
}

#[tokio::main]
async fn main() -> Result<()> {
    Database::add_connection("MYSQL", MySQL::new("postgresql://postgres:test123@localhost:5111/lucas11776").await?);
    Database::add_connection("SQLITE", SQLite::new("database.sqlite").await?);
    Database::add_connection("POSTGRES", Postgres::new("postgresql://postgres:test123@localhost:5111/lucas11776").await?);
    Ok(())
}