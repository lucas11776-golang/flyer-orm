# Flyer - ORM Framework


## Information

Flyer-ORM is a powerful and lightweight Object-Relational Mapping (ORM) framework for Rust, designed to make database interactions intuitive and efficient. It provides a fluent query builder and supports multiple database backends with built-in connection management.

### Supports

- MySQL
- PostgreSQL
- SQLite


## Getting Started

### Key Features:

- Fluent Query Builder
- Connection Management
- Database Migrations (Raw SQL Execution)
- Transactions Support
- Pagination
- Raw SQL Query Support
- Type-safe results with `sqlx` and `serde`


## Getting with Flyer-ORM

First create a new project using command:

```sh
cargo new example
```

After running the command add `flyer-orm` to your project using command:

```sh
cargo add flyer-orm
```

Also add `tokio`, `sqlx` and `serde` as they are commonly used with Flyer-ORM:

```sh
cargo add tokio --features full
cargo add sqlx --features "runtime-tokio-rustls"
cargo add serde --features derive
```

### Connection Management

Flyer-ORM allows you to manage multiple database connections easily.

```rust
use flyer_orm::{DB, databases::sqlite::SQLite};

#[tokio::main]
async fn main() {
    // Add a named connection
    DB::add("main", "sqlite:database.sqlite");

    // Retrieve a database instance by name
    let db = DB::db::<SQLite>("main").await;
    
    // Or use a URL directly without registration
    let db = DB::db_with_url::<SQLite>("sqlite::memory:").await;
}
```

### Basic Queries

Flyer-ORM provides a fluent interface for building SQL queries.

```rust
use flyer_orm::{Database, databases::sqlite::SQLite, query::Order};
use serde::Serialize;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
}

#[tokio::main]
async fn main() {
    let db = Database::<SQLite>::new(":memory:").await;

    // Fetch users with filters
    let users = db.query("users")
        .select(vec!["id", "name", "email"])
        .r#where("id", ">", 10)
        .and_where("name", "LIKE", "%John%")
        .order_by("id", Order::DESC)
        .limit(10)
        .all::<User>()
        .await
        .unwrap();

    println!("Users: {:?}", users);
}
```

### Joins

You can easily perform various types of joins including Inner, Left, Right, Full Outer, and Cross Joins.

```rust
async fn get_user_projects(db: &Database<SQLite>) {
    let results = db.query("users")
        .select(vec!["users.name", "projects.title"])
        .join("projects", "projects.user_id", "=", "users.id")
        .all::<UserProject>() // Assuming UserProject struct matches the results
        .await
        .unwrap();
}
```

### Insert

Inserting data is straightforward. You can insert raw data or insert and retrieve the resulting object.

```rust
#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct User {
    pub uuid: String,
    pub first_name: String,
    pub email: String,
}

async fn create_user(db: &Database<SQLite>) {
    // Insert and return the record mapped to a struct
    let user = db.query("users")
        .insert_as::<User>(vec!["uuid", "first_name", "email"])
        .bind(uuid::Uuid::new_v4().to_string())
        .bind("Jane Doe")
        .bind("jane@example.com")
        .execute()
        .await
        .unwrap();

    // Basic insert without return mapping
    db.query("projects")
        .insert(vec!["title", "user_id"])
        .bind("New Website")
        .bind(1)
        .execute()
        .await
        .unwrap();
}
```

### Update

```rust
async fn update_user(db: &Database<SQLite>) {
    db.query("users")
        .update(vec!["first_name"])
        .bind("Updated Name")
        .r#where("uuid", "=", "some-uuid")
        .execute()
        .await
        .unwrap();
}
```

### Delete

```rust
async fn delete_user(db: &Database<SQLite>) {
    db.query("users")
        .r#where("id", "=", 1)
        .delete()
        .await
        .unwrap();
}
```

### Pagination

Flyer-ORM handles complex pagination with ease, returning total counts and current page items.

```rust
async fn list_paginated_users(db: &Database<SQLite>) {
    let pagination = db.query("users")
        .paginate::<User>(10, 1) // 10 items per page, page 1
        .await
        .unwrap();

    println!("Total: {}", pagination.total);
    println!("Current Page: {}", pagination.page);
    println!("Items: {:?}", pagination.items);
}
```

### Transactions

Execute multiple operations safely within a database transaction.

```rust
async fn safe_operation(db: &Database<SQLite>) {
    let transaction = db.transaction().await.unwrap();
    
    // Perform operations...
    // Note: Transaction support is integrated with the underlying executor.
    
    transaction.commit().await.unwrap();
    // or
    // transaction.rollback().await.unwrap();
}
```

### Raw SQL Execution

To allow easy change of database you can use connection pattern.

```rust
use std::env;

use anyhow::Result;
use flyer_orm::{
    Database,
    databases::{mysql::MySQL, postgres::Postgres, sqlite::SQLite}
};
use serde::Serialize;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct User {
    pub uuid: String,
    pub created_at: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

const USERS_TABLE_SCHEME: &'static str = "CREATE TABLE users (
 `uuid` VARCHAR(65535) PRIMARY KEY NOT NULL UNIQUE,
 `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 `first_name` VARCHAR(65535),
 `last_name` VARCHAR(65535),
 `email` VARCHAR(65535) NOT NULL,
 `password` VARCHAR(65535) NOT NULL
)";

pub struct Connection;

impl Connection {
    // TODO: Using SQLite
    pub async fn db() -> Database<SQLite> {
        return Database::<SQLite>::new(":memory:").await;
    }

    // // TODO: Using Postgres
    // pub async fn db() -> Database<Postgres> {
    //     return Database::<Postgres>::new(&env::var("DATABASE_URL").unwrap()).await;
    // }

    // // TODO: Using MySQL
    // pub async fn db() -> Database<MySQL> {
    //     return Database::<MySQL>::new(&env::var("DATABASE_URL").unwrap()).await;
    // }
}

#[tokio::main]
async fn main() -> Result<()> {
    let db = Connection::db().await;

    // Migrate database with users table
    if let Err(err) = db.execute(USERS_TABLE_SCHEME).await {
        panic!("Error: {:?}", err)
    }

    let user = db.query("users")
        .insert_as::<User>(vec!["uuid", "first_name", "last_name", "email", "password"])
        .bind(uuid::Uuid::new_v4().to_string())
        .bind("Jeo")
        .bind("doe")
        .bind("jeo@doe.com")
        .bind("test@123")
        .execute()
        .await
        .unwrap();

    println!("User: {:?}", user);

    return Ok(());
}
```

### Utilities

Check for existence or count records quickly.

```rust
async fn utilities(db: &Database<SQLite>) {
    let count = db.query("users").count().await.unwrap();
    let exists = db.query("users").r#where("id", "=", 1).exists().await.unwrap();
    
    // Preview the generated SQL
    let sql = db.query("users").r#where("status", "=", "active").to_sql().unwrap();
    println!("Generated SQL: {}", sql);
}
```