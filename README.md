# Flyer ORM 🚀

A simple, lightweight, fast, and asynchronous ORM for Rust built on top of SQLx. Flyer ORM supports **PostgreSQL**, **MySQL**, and **SQLite** with an intuitive and expressive fluent query builder. 

This document provides a comprehensive and detailed guide for using Flyer ORM, structured around the actual runnable examples found under the `examples/` directory.

---

## Table of Contents

- [Core Architecture Concepts](#core-architecture-concepts)
- [Example 1: Connection Setup & Configuration (`01_setup.rs`)](#example-1-connection-setup--configuration-01_setuprs)
- [Example 2: Creating Tables & Inserting Records (`02_insert.rs`)](#example-2-creating-tables--inserting-records-02_insertrs)
- [Example 3: Querying, Filtering & Parameterized Raw SQL (`03_query.rs`)](#example-3-querying-filtering--parameterized-raw-sql-03_queryrs)
- [Example 4: Updating Records (`04_update.rs`)](#example-4-updating-records-04_updaters)
- [Example 5: Deleting Records & Counting (`05_delete.rs`)](#example-5-deleting-records--counting-05_deleters)
- [Example 6: Native Query Pagination (`06_pagination.rs`)](#example-6-native-query-pagination-06_paginationrs)

---

## Core Architecture Concepts

Flyer ORM revolves around a few key abstractions:
* **`Database<E>`**: The primary entry point for executing database operations. It acts as a handle to a connection pool. It is generic over an executor type `E` (such as `Postgres`, `SQLite`, or `MySQL`).
* **`Executor`**: A trait representing a database backend. Connections are initialized with executors and held in a global registry.
* **`Entity`**: A trait (and derive macro) used to map database tables to Rust structures. It enables automatic deserialization of query results into your custom structs.

---

## Example 1: Connection Setup & Configuration (`01_setup.rs`)

This example demonstrates how to configure, register, and retrieve multi-backend database connections within your application.

### Runnable Source Code

```rust
use flyer_orm::{Database, Executor, MySQL, Postgres, SQLite, Result};

/// Helper function for simple database usage. You may also pass the database name as
/// a parameter if you have multiple database connections (such as "orders", "payments", etc.).
/// 
/// ```rust,no_run
/// pub fn db(connection: &str) -> Database<Postgres> {
///     Database::<Postgres>::connection(connection)
/// } 
/// ```
pub fn db() -> Database<Postgres> {
    Database::<Postgres>::connection("SQLITE")
}

#[tokio::main]
async fn main() -> Result<()> {
    // Register a MySQL connection pool
    Database::add_connection(
        "MYSQL", 
        MySQL::new("postgresql://postgres:test123@localhost:5111/lucas11776").await?
    );

    // Register a SQLite connection pool
    Database::add_connection(
        "SQLITE", 
        SQLite::new("database.sqlite").await?
    );

    // Register a Postgres connection pool
    Database::add_connection(
        "POSTGRES", 
        Postgres::new("postgresql://postgres:test123@localhost:5111/lucas11776").await?
    );

    Ok(())
}
```

### Detailed API Breakdown

1. **`Database::add_connection(name, executor)`**:
   - Registers a database connection pool globally under a specific name (e.g., `"SQLITE"`, `"POSTGRES"`).
   - This allows you to manage multiple heterogeneous databases simultaneously within a single binary.

2. **`Database::<E>::connection(name)`**:
   - Retrieves the registered database connection of type `E`.
   - **Type Safety Warning**: The executor type parameter `E` (e.g. `Postgres`, `SQLite`) must match the actual connection type registered under that name. If there is a mismatch, the function will panic with `"Connection '<name>' not found or type mismatch"`.

### Running the Example

Run this example using Cargo:
```bash
cargo run --example 01_setup
```

---

## Example 2: Creating Tables & Inserting Records (`02_insert.rs`)

This example shows how to perform table migrations (raw execution) and insert data into a table, while instantly returning the fully structured inserted entity.

### Runnable Source Code

```rust
use flyer_orm::{Database, Entity, Executor, Postgres, Result, SQLite};
use serde::Serialize;

const USERS_TABLE_SCHEME: &'static str = "
    CREATE TABLE users (
        `uuid` VARCHAR(65535) PRIMARY KEY NOT NULL UNIQUE,
        `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        `first_name` VARCHAR(65535),
        `last_name` VARCHAR(65535),
        `email` VARCHAR(65535) NOT NULL,
        `password` VARCHAR(65535) NOT NULL
    );
";

#[derive(Debug, Entity, Serialize)]
pub struct User {
    pub uuid: String,
    pub created_at: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

pub fn db() -> Database<Postgres> {
    Database::<Postgres>::connection("default")
}

#[tokio::main]
pub async fn main() -> Result<()> {
    // Add an in-memory SQLite connection
    Database::add_connection("default", SQLite::new(":memory:").await?);

    // 1. Run raw SQL to create the table structure (Migration)
    let migrated = db()
        .raw(USERS_TABLE_SCHEME)
        .execute()
        .await;

    if let Err(err) = migrated {
        panic!("Migration Error: {:?}", err);
    }

    // 2. Perform fluent insert and retrieve the resulting entity
    let result = db()
        .insert("users")
        .bind("uuid", uuid::Uuid::new_v4().to_string())
        .bind("first_name", "Jeo".to_string())
        .bind("last_name", "Deo".to_string())
        .bind("email", "jeo@deo.com".to_string())
        .bind("password", "test@123".to_string())
        .execute_as::<User>()
        .await;

    if let Err(err) = result {
        panic!("Get User Error: {:?}", err);
    }

    let user = result.unwrap();

    println!("\r\n\r\n\r\n\r\nUser: {:?}\r\n\r\n\r\n\r\n", user);

    Ok(())
}
```

### Detailed API Breakdown

1. **`#[derive(Entity)]`**:
   - Marks your struct as a database entity, allowing Flyer ORM's serialization and mapping engines to automatically bind table rows to struct fields.

2. **`db().raw(sql).execute()`**:
   - Executes raw SQL statements directly on the database (ideal for migrations, manual DDL, or custom commands).
   - Returns a `Result<QueryResult>` indicating the success of the execution.

3. **`db().insert(table_name)`**:
   - Initiates the fluent insert builder targeting the specified table.

4. **`.bind(column_name, value)`**:
   - Binds a specific value to a column. Under the hood, this performs safe parameterized bindings to prevent SQL Injection attacks.

5. **`.execute_as::<T>()`**:
   - Executes the insert operation, automatically queries the newly inserted row, maps it, and returns the deserialized entity struct of type `T`.

### Running the Example

Run this example using Cargo:
```bash
cargo run --example 02_insert
```

---

## Example 3: Querying, Filtering & Parameterized Raw SQL (`03_query.rs`)

This example details how to build structured where conditions, handle grouped boolean logic, fetch lists of records, and execute parameterized raw queries.

### Runnable Source Code

```rust
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

    // 1. Fluent Query with Grouped Where conditions
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

    // 2. Parameterized Raw Write Query
    let product_name = String::from("Butter");

    db()
        .raw("INSERT INTO products (name, price) VALUES ($1, $2)")
        .bind(product_name.clone())
        .bind(150)
        .execute()
        .await
        .unwrap();

    // 3. Simple Fluent Query with single where clause
    let product = db()
        .query("products")
        .r#where("name", "=", product_name)
        .first::<Product>()
        .await
        .unwrap();

    println!("\r\nINSERTED PRODUCT {:?}", product);

    Ok(())
}
```

### Detailed API Breakdown

1. **`db().query(table_name)`**:
   - Initiates a fluent read query builder targeting the specified table.

2. **`.where_group(|group| { ... })`**:
   - Creates a logical group of filters (rendered as parenthesized logical blocks in SQL, e.g., `(price >= 20 AND price <= 40)`).
   - Inside the closure, you build conditions using `group.r#where(...)`, `group.and_where(...)`, or `group.or_where(...)`.

3. **`.r#where(column, operator, value)`**:
   - Adds a single conditional filter mapping `column operator value` (e.g., `"name", "=", "Butter"`).

4. **`.all::<T>()`**:
   - Executes the query and returns all matching rows as a vector of the specified Entity type (`Vec<T>`).

5. **`db().raw(sql).bind(val)`**:
   - Constructs a raw SQL query with placeholders (e.g. `$1`, `$2`), and maps actual variables to those placeholders in order using chained `.bind(val)` calls.

6. **`.first::<T>()`**:
   - Executes the query and returns the first matching record as `Result<T>`.

### Running the Example

Run this example using Cargo:
```bash
cargo run --example 03_query
```

---

## Example 4: Updating Records (`04_update.rs`)

This example demonstrates how to filter specific records and update their values within the database.

### Runnable Source Code

```rust
use flyer_orm::{Database, Entity, Executor, Postgres, Result, SQLite};
use serde::Serialize;

#[derive(Debug, Entity, Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub status: String,
}

pub fn db() -> Database<Postgres> {
    Database::<Postgres>::connection("default")
}

#[tokio::main]
pub async fn main() -> Result<()> {
    Database::add_connection("default", SQLite::new(":memory:").await?);

    // Set up schema and initial data
    db()
        .raw("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, status TEXT)")
        .execute()
        .await
        .unwrap();

    let user = db()
        .insert("users")
        .bind("name", String::from("John"))
        .bind("status", String::from("inactive"))
        .execute_as::<User>()
        .await
        .unwrap();

    println!("\r\n\r\nUser created with status: {}", user.status);

    // Fluent Update Operation
    db()
        .update("users")
        .r#where("id", "=", user.id)
        .bind("status", String::from("active"))
        .execute()
        .await
        .unwrap();

    // Query to verify update
    let user = db()
        .query("users")
        .r#where("id", "=", user.id)
        .first::<User>()
        .await
        .unwrap();

    println!("User updated with status: {}\r\n\r\n", user.status);

    Ok(())
}
```

### Detailed API Breakdown

1. **`db().update(table_name)`**:
   - Initiates an update builder targeting the specified table.

2. **`.r#where(...)`**:
   - Restricts the scope of the update statement to records matching specific criteria (highly recommended to prevent updating all rows in the table!).

3. **`.bind(column_name, new_value)`**:
   - Specifies which columns are updated, and binds their new target values in a parameterized fashion.

4. **`.execute()`**:
   - Executes the finalized update query against the database.

### Running the Example

Run this example using Cargo:
```bash
cargo run --example 04_update
```

---

## Example 5: Deleting Records & Counting (`05_delete.rs`)

This example covers deleting rows using filter conditions and retrieving the total count of matching rows in a table.

### Runnable Source Code

```rust
use flyer_orm::{Database, Entity, Executor, Postgres, Result, SQLite};
use serde::Serialize;

#[derive(Debug, Entity, Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
}

pub fn db() -> Database<Postgres> {
    Database::<Postgres>::connection("default")
}

#[tokio::main]
pub async fn main() -> Result<()> {
    Database::add_connection("default", SQLite::new(":memory:").await?);

    // Set up schema and initial data
    db()
        .raw("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, email TEXT)")
        .execute()
        .await
        .unwrap();

    /* INSERT DATA */
    db()
        .insert("users")
        .bind("name", String::from("John"))
        .bind("email", String::from("john@deo.com"))
        .execute()
        .await
        .unwrap();

    db()
        .insert("users")
        .bind("name", String::from("Jane"))
        .bind("email", String::from("jane@doe.com"))
        .execute()
        .await
        .unwrap();

    println!("\r\n\r\nNumber of users: {}", db()
        .query("users").count().await.unwrap());

    // Fluent Delete Operation
    db()
        .delete("users")
        .r#where("email", "=", String::from("john@deo.com"))
        .execute()
        .await
        .unwrap();

    // Verification via count
    let total = db()
        .query("users")
        .count()
        .await
        .unwrap(); 

    println!("Number of users: {}\r\n\r\n", total);

    Ok(())
}
```

### Detailed API Breakdown

1. **`db().delete(table_name)`**:
   - Starts a delete query builder targeting the specified table.

2. **`.r#where(...)`**:
   - Constrains the delete operation to rows matching the criteria.

3. **`db().query(table_name).count()`**:
   - Computes and returns the total number of records matching the query constraints as a `Result<i64>` (equivalent to running `SELECT COUNT(*) FROM table`).

### Running the Example

Run this example using Cargo:
```bash
cargo run --example 05_delete
```

---

## Example 6: Native Query Pagination (`06_pagination.rs`)

This example demonstrates native database pagination support, including total item counting, page navigation, and item mapping.

### Runnable Source Code

```rust
use flyer_orm::{Database, Entity, Executor, Postgres, Result, SQLite};
use serde::Serialize;

#[derive(Debug, Entity, Serialize)]
pub struct Post {
    pub id: i64,
    pub title: String,
}

const POSTS_TABLE_SCHEMA: &'static str = "
    CREATE TABLE posts (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        title VARCHAR(255) NOT NULL
    );

    INSERT INTO posts 
        (title)
    VALUES
        ('Post 1'),
        ('Post 2'),
        ('Post 3'),
        ('Post 4'),
        ('Post 5');
";

pub fn db() -> Database<Postgres> {
    Database::<Postgres>::connection("default")
}

#[tokio::main]
pub async fn main() -> Result<()> {
    Database::add_connection("default", SQLite::new(":memory:").await?);

    let migrated = db()
        .raw(POSTS_TABLE_SCHEMA)
        .execute()
        .await;

    if let Err(err) = migrated {
        panic!("Migration Error: {:?}", err);
    }

    // Retrieve page 2 with a limit of 2 items per page
    let result = db()
        .query("posts")
        .paginate::<Post>(2, 2)
        .await;

    if let Err(err) = result {
        panic!("Get User Error: {:?}", err);
    }

    let pagination = result.unwrap();

    println!("Total posts: {}", pagination.total);
    println!("Current page: {}", pagination.page);
    println!("Items on this page:");

    for post in pagination.items {
        println!(" - [{}]: {}", post.id, post.title);
    }

    Ok(())
}
```

### Detailed API Breakdown

1. **`db().query(table_name).paginate::<T>(page, per_page)`**:
   - Automatically handles the pagination logic for table rows.
   - Calculates the appropriate SQL `LIMIT` and `OFFSET` bounds dynamically based on the requested page and limits.
   - It also fires a concurrent (or sequence) count query under the hood to calculate the total number of matches available.

2. **Pagination Response Struct**:
   Calling `paginate::<T>()` returns a `Pagination<T>` structure containing:
   - **`total`**: `i64` — The total number of items matching the query in the database.
   - **`page`**: `i64` — The current active page index requested.
   - **`items`**: `Vec<T>` — A vector of mapped entities belonging to the current page.

### Running the Example

Run this example using Cargo:
```bash
cargo run --example 06_pagination
```
