use anyhow::Result;
use flyer_orm::{
    Database,
    databases::sqlite::SQLite
};

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to SQLite in-memory database
    let db = Database::<SQLite>::new(":memory:").await;

    // Create a simple table
    db.raw_query("CREATE TABLE accounts (id INTEGER PRIMARY KEY, balance INTEGER)").execute().await?;
    db.raw_query("INSERT INTO accounts (balance) VALUES (100), (200)").execute().await?;

    println!("Initial balances set.");

    // Start a transaction
    let tx = db.transaction().await?;
    
    // In current version, transaction is integrated with the executor, 
    // but the query builder is not yet fully transaction-aware. 
    // You can use raw SQL through execute() on the database or manage it manually.
    
    // Note: Transaction management and its connection to the query builder 
    // is a work in progress. For now, we can commit or rollback.
    
    println!("Transaction started...");
    
    // Example: committing the transaction
    tx.commit().await?;
    
    println!("Transaction committed successfully.");

    return Ok(());
}
