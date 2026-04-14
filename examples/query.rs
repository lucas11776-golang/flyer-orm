use anyhow::Result;
use flyer_orm::{
    Database,
    databases::{mysql::MySQL, postgres::Postgres, sqlite::SQLite}
};
use serde::Serialize;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Product {
    pub id: u64,
    pub created_at: String,
    pub name: String,
    pub price: f32,
}

const USERS_TABLE_SCHEME: &'static str = "
CREATE TABLE products (
 `id` INTEGER AUTO_INCREMENT PRIMARY KEY NOT NULL UNIQUE,
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

pub struct Connection;

impl Connection {
    // TODO: Using SQLite
    pub async fn db() -> Database<SQLite> {
        return Database::<SQLite>::new(":memory:").await;
    }

    // // TODO: Using Postgres
    // pub async fn db() -> Database<Postgres> {
    //     return Database::<Postgres>::new("postgresql://user:@127.0.0.1:8080/db?").await;
    // }

    // // TODO: Using MySQL
    // pub async fn db() -> Database<MySQL> {
    //     return Database::<MySQL>::new("mysql://user:@127.0.0.1:8080/db?").await;
    // }
}

#[tokio::main]
async fn main() -> Result<()> {
    let db = Connection::db().await;

    // Migrate database with users table
    if let Err(err) = db.execute(USERS_TABLE_SCHEME).await {
        panic!("Error: {:?}", err)
    }

    let products_range_r10_to_r30 = db.query("products")
        .r#where("price", ">=", 20)
        .and_where("price", "=<", 40)
        .all::<Product>()
        .await
        .unwrap();


    println!("User: {:?}", products_range_r10_to_r30);

    return Ok(());
}