use flyer_orm::Result;
use flyer_orm::{Connection, Executor, MySQL, Postgres, SQLite};

#[tokio::main]
async fn main() -> Result<()> {
    Connection::add("mysql", MySQL::new(":PATH:").await);
    Connection::add("postgres", Postgres::new(":URL:").await);
    Connection::add("sqlite", SQLite::new(":URL:").await);

    return Ok(());
}