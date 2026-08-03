use anyhow::Result;
use flyer_orm::{Connection, Executor, Query, SQLite, query::raw::Raw};
use flyer_orm::Entity;
use serde::Serialize;


#[derive(Debug, Entity, Serialize)]
pub struct Subscription {
    pub name: String,
    pub email: String,
}

pub struct Database;

impl Database {
    pub fn query<'q>(table: impl Into<String>) -> Query<'q, SQLite> {
        Connection::query("default").table(table)
    }

    pub fn raw<'q>(sql: impl Into<String>) -> Raw<'q, SQLite> {
        Connection::raw("default", sql)
    }
}
    
// In current version, transaction is integrated with the executor, 
// but the query builder is not yet fully transaction-aware. 
// You can use raw SQL through execute() on the database or manage it manually.

// Note: Transaction management and its connection to the query builder 
// is a work in progress. For now, we can commit or rollback.
#[tokio::main]
async fn main() -> Result<()> {
    Connection::add("default", SQLite::new(":memory:").await);

    // Set up schema and initial data
    Database::raw("CREATE TABLE subscriptions (name TEXT, email TEXT)")
        .execute()
        .await
        .unwrap();

    let mut query = Database::query("subscriptions");

    /* Commit Transaction */
    let transaction = query
        .transaction()
        .await
        .unwrap();

    query
        .insert()
        .bind("name", String::from("Jeo"))
        .bind("email", String::from("jeo@deo.com"))
        .execute()
        .await
        .unwrap();

    transaction
        .commit()
        .await
        .unwrap();

    /* Rollback Transaction */
    let transaction = query
        .transaction()
        .await
        .unwrap();

    query
        .insert()
        .bind("name", String::from("Jane"))
        .bind("email", String::from("jane@deo.com"))
        .execute()
        .await
        .unwrap();

    transaction
        .rollback()
        .await
        .unwrap();

    println!("Users inserted: {}", query.count().await.unwrap());
    
    return Ok(());
}