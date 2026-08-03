# Flyer ORM

A simple and intuitive ORM for Rust, supporting SQLite (and potentially others) with a clean query builder and transaction support.

## Getting Started

### 1. Define your Database Connection

First, you need to register a connection. You can have multiple named connections.

```rust
use flyer_orm::{Connection, SQLite};

#[tokio::main]
async fn main() {
    // Register a default SQLite connection
    Connection::add("default", SQLite::new(":memory:").await);
}
```

### 2. Define your Entities

Map your database tables to Rust structs using the `Entity` and `Serialize` derives.

```rust
use flyer_orm::Entity;
use serde::Serialize;

#[derive(Debug, Entity, Serialize)]
pub struct User {
    pub uuid: String,
    pub name: String,
    pub email: String,
}
```

### 3. Recommended: Create a Database Helper

A common pattern is to create a small helper struct to simplify query access.

```rust
use flyer_orm::{Connection, Query, SQLite, query::raw::Raw};

pub struct Database;

impl Database {
    pub fn query<'q>(table: impl Into<String>) -> Query<'q, SQLite> {
        Connection::query("default").table(table)
    }

    pub fn raw<'q>(sql: impl Into<String>) -> Raw<'q, SQLite> {
        Connection::raw("default", sql)
    }
}
```

---

## CRUD Operations

### Create (Insert)

You can insert data and retrieve the created entity in one go.

```rust
let user = Database::query("users")
    .insert()
    .bind("uuid", uuid::Uuid::new_v4().to_string())
    .bind("name", "John Doe".to_string())
    .bind("email", "john@example.com".to_string())
    .execute_as::<User>()
    .await?;
```

### Read (Query)

The query builder supports standard SQL operations.

```rust
// Fetch all matching records
let active_users = Database::query("users")
    .r#where("status", "=", "active")
    .all::<User>()
    .await?;

// Fetch the first record
let user = Database::query("users")
    .r#where("id", "=", 1)
    .first::<User>()
    .await?;

// Count records
let count = Database::query("users").count().await?;
```

### Update

Update specific records using filters.

```rust
Database::query("users")
    .update()
    .r#where("id", "=", 1)
    .bind("name", "Jane Doe".to_string())
    .execute()
    .await?;
```

### Delete

Delete records matching your criteria.

```rust
Database::query("users")
    .delete()
    .r#where("email", "=", "john@example.com")
    .execute()
    .await?;
```

---

## Advanced Features

### Raw SQL

For migrations or complex queries, use raw SQL execution.

```rust
// Migration
Database::raw("CREATE TABLE products (id INTEGER PRIMARY KEY, name TEXT)")
    .execute()
    .await?;

// Parameterized Raw Query
let result = Database::raw("INSERT INTO products (name) VALUES ($1)")
    .bind("Bread".to_string())
    .execute()
    .await?;
```

### Pagination

Easily paginate your result sets.

```rust
// paginate::<EntityType>(page_number, items_per_page)
let pagination = Database::query("posts")
    .paginate::<Post>(1, 10)
    .await?;

println!("Total records: {}", pagination.total);
for post in pagination.items {
    println!("Title: {}", post.title);
}
```

### Transactions

Execute multiple operations within an atomic transaction.

```rust
let mut query = Database::query("subscriptions");

let transaction = query.transaction().await?;

query.insert()
    .bind("user_id", 1)
    .execute()
    .await?;

// Commit to save changes
transaction.commit().await?;

// Or rollback to discard
// transaction.rollback().await?;
```

---

## Examples

Check the `examples/` directory for full, runnable examples:
- `01_insert.rs`: Setup and Insertion
- `02_query.rs`: Selecting and Filtering
- `03_update.rs`: Updating Records
- `04_delete.rs`: Deletion and Counting
- `05_pagination.rs`: Paginating Results
- `06_transaction.rs`: Using Transactions
