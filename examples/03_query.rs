use flyer_orm::{Database, Entity, Executor, Postgres, Result, SQLite};
use serde::Serialize;

#[derive(Debug, Entity, Serialize)]
pub struct Product {
    pub id: i64,
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

    INSERT INTO products
        (id, name, price)
    VALUES
        (1, 'Bread', 20),
        (2, 'Milk', 25),
        (3, 'Sugar', 45),
        (4, 'Eggs', 65);
";

pub fn db() -> Database<Postgres> {
    Database::<Postgres>::connection("default")
}

#[tokio::main]
pub async fn main() -> Result<()> {
    Database::add_connection("default", SQLite::new(":memory:").await?);

    let migrated = db()
        .raw(USERS_TABLE_SCHEME)
        .execute()
        .await;

    if let Err(err) = migrated {
        panic!("Migration Error: {:?}", err);
    }
    let products_range_r10_to_r30 = db()
        .query("products")
        .where_group(|group| {
            group
                .r#where("price", ">=", 20)
                .and_where("price", "<=", 40);
        })
        .all::<Product>()
        .await
        .unwrap();

    for product in products_range_r10_to_r30 {
        println!("Product: {:?}", product);
    }

    // Raw Query
    let product_name = String::from("Butter");

    db()
        .raw("INSERT INTO products (name, price) VALUES ($1, $2)")
        .bind(product_name.clone())
        .bind(150)
        .execute()
        .await
        .unwrap();

    let product = db()
        .query("products")
        .r#where("name", "=", product_name)
        .first::<Product>()
        .await
        .unwrap();

    println!("\r\nINSERTED PRODUCT {:?}", product);

    Ok(())
}