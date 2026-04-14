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

pub struct Connection;

impl Connection {
    pub async fn db() -> Database<SQLite> {
        return Database::<SQLite>::new(":memory:").await;
    }
}

#[tokio::main]
async fn main() {
    let db = Connection::db().await;

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

For complex queries or migrations, you can execute raw SQL directly. The `execute` method returns a `QueryResult` which provides information about the operation.

```rust
use flyer_orm::query::QueryResult;

async fn run_migrations(db: &Database<SQLite>) {
    let schema = "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)";
    let result = db.execute(schema).await.unwrap();
    
    println!("Rows affected: {}", result.rows_affected());
}

async fn raw_query(db: &Database<SQLite>) {
    let users: Vec<User> = db.query("users")
        .query_all::<User, String>("SELECT * FROM users WHERE email = ?", vec!["test@test.com".to_string()])
        .await
        .unwrap();
}
```

### Connection Pattern

The recommended pattern for managing your database instance is to use a `Connection` struct. This makes it easy to switch between different database backends (MySQL, Postgres, SQLite) by simply changing the implementation of the `db()` function.

```rust
use flyer_orm::{Database, databases::{mysql::MySQL, postgres::Postgres, sqlite::SQLite}};

pub struct Connection;

impl Connection {
    // SQLite Implementation
    pub async fn db() -> Database<SQLite> {
        return Database::<SQLite>::new(":memory:").await;
    }

    // Postgres Implementation
    // pub async fn db() -> Database<Postgres> {
    //     return Database::<Postgres>::new("postgresql://user:pass@127.0.0.1:5432/db").await;
    // }

    // MySQL Implementation
    // pub async fn db() -> Database<MySQL> {
    //     return Database::<MySQL>::new("mysql://user:pass@127.0.0.1:3306/db").await;
    // }
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