use anyhow::Result;
use flyer_orm::types::QueryResult;
use flyer_orm::{Connection, Executor, Query, SQLite, query::raw::Raw};
use flyer_orm::Entity;
use serde::Serialize;

#[derive(Debug, Entity, Serialize)]
pub struct Product {
    pub id: u64,
    pub created_at: String,
    pub name: String,
    pub price: f32,
}

const USERS_TABLE_SCHEME: &'static str = "
CREATE TABLE products (
 `id` INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
 `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 `name` VARCHAR(65535) NOT NULL,
 `price` FLOAT NOT NULL
);

INSERT INTO products (id, name, price)
VALUES
    (1, 'Bread', 20),
    (2, 'Milk', 25),
    (3, 'Sugar', 45),
    (4, 'Eggs', 65);
";

pub struct Database;

impl Database {
    pub fn query<'q>(table: impl Into<String>) -> Query<'q, SQLite> {
        Connection::query("default").table(table)
    }

    pub fn raw<'q>(sql: impl Into<String>) -> Raw<'q, SQLite> {
        Connection::raw("default", sql)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    Connection::add("default", SQLite::new(":memory:").await);

    let migrated = Database::raw(USERS_TABLE_SCHEME)
        .execute()
        .await;

    if let Err(err) = migrated {
        panic!("Migration Error: {:?}", err);
    }

    let products_range_r10_to_r30 = Database::query("products")
        .r#where("price", ">=", 20)
        .and_where("price", "<=", 40)
        .all::<Product>()
        .await
        .unwrap();

    println!("\r\n\r\nUser: {:?}", products_range_r10_to_r30);

    // Raw Query
    let result = Database::raw("INSERT INTO products (name, price) VALUES ($1, $2)")
        .bind("Butter".to_owned())
        .bind(150)
        .execute()
        .await
        .unwrap();

    println!("\r\n\r\nQUERY RESULT -> {:?} -> AFFECTED -> {:?}", result.last_inserted(), result.rows_affected());

    return Ok(());
}