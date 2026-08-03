# Flyer ORM 🚀

A lightweight, asynchronous, and intuitive ORM for Rust. Designed to be simple to use while providing powerful features like a type-safe query builder, easy pagination, and transaction support.

Currently supports **SQLite**, with a focus on ease of use and developer productivity.

---

## 🛠 Getting Started

### 1. Connection Registration

Register your database connections at the start of your application. You can manage multiple named connections.

```rust
use flyer_orm::{Connection, SQLite};

#[tokio::main]
async fn main() {
    // ":memory:" creates an in-memory database for testing
    // You can also use a file path like "my_database.sqlite"
    Connection::add("default", SQLite::new(":memory:").await);
}
```

### 2. Defining Your Entities

Entities are simple Rust structs that map to your database tables. Use the `Entity` and `Serialize` derives.

```rust
use flyer_orm::Entity;
use serde::Serialize;

#[derive(Debug, Entity, Serialize)]
pub struct User {
    pub id: i64,          // Auto-increment primary key
    pub name: String,
    pub email: String,
    pub status: String,
}
```

### 3. Recommended Pattern: The Database Helper

To make your code cleaner, it's recommended to create a helper struct to avoid repeating connection names.

```rust
use flyer_orm::{Connection, Query, SQLite, query::raw::Raw};

pub struct DB;

impl DB {
    /// Start a new query on a specific table
    pub fn query<'q>(table: impl Into<String>) -> Query<'q, SQLite> {
        Connection::query("default").table(table)
    }

    /// Execute raw SQL queries
    pub fn raw<'q>(sql: impl Into<String>) -> Raw<'q, SQLite> {
        Connection::raw("default", sql)
    }
}
```

---

## 🔍 Querying Data

### Select All or One
```rust
// Fetch all active users
let users = DB::query("users")
    .r#where("status", "=", "active")
    .all::<User>()
    .await?;

// Fetch the first user by email
let user = DB::query("users")
    .r#where("email", "=", "john@example.com")
    .first::<User>()
    .await?;
```

### Complex Filtering (AND/OR)
```rust
let results = DB::query("products")
    .r#where("price", ">=", 10)
    .and_where("price", "<=", 50)
    .and_where("category", "=", "electronics")
    .all::<Product>()
    .await?;
```

### Counting Records
```rust
let total_users = DB::query("users").count().await?;
```

---

## ✍️ Modifying Data

### Insertion
You can insert data and immediately retrieve the resulting object.

```rust
let new_user = DB::query("users")
    .insert()
    .bind("name", "Alice".to_string())
    .bind("email", "alice@example.com".to_string())
    .bind("status", "pending".to_string())
    .execute_as::<User>() // Automatically maps to the User struct
    .await?;
```

### Updating
```rust
DB::query("users")
    .update()
    .r#where("id", "=", 1)
    .bind("status", "active".to_string())
    .execute()
    .await?;
```

### Deleting
```rust
DB::query("users")
    .delete()
    .r#where("status", "=", "banned")
    .execute()
    .await?;
```

---

## 🚀 Advanced Features

### 📄 Pagination Made Easy
Flyer ORM handles the math for you.

```rust
// paginate::<EntityType>(page_number, items_per_page)
let pagination = DB::query("posts")
    .paginate::<Post>(1, 15) // Page 1, 15 items per page
    .await?;

println!("Current Page: {}", pagination.page);
println!("Total Pages: {}", pagination.total_pages());
println!("Total Items: {}", pagination.total);

for post in pagination.items {
    println!("- {}", post.title);
}
```

### ⚡ Raw SQL Execution
Sometimes you just need to write SQL.

```rust
// Executing a command (DDL or DML)
DB::raw("CREATE TABLE IF NOT EXISTS logs (message TEXT)").execute().await?;

// Executing with parameters (prevents SQL injection)
DB::raw("INSERT INTO products (name, price) VALUES ($1, $2)")
    .bind("Coffee".to_string())
    .bind(4.99)
    .execute()
    .await?;
```

### 🔐 Transactions
Ensure data integrity with atomic operations.

```rust
let mut query = DB::query("accounts");
let tx = query.transaction().await?;

// All operations here are part of the transaction
query.update().r#where("id", "=", 1).bind("balance", 100).execute().await?;
query.update().r#where("id", "=", 2).bind("balance", 200).execute().await?;

// Commit to save, or it will automatically rollback if dropped
tx.commit().await?;
```

---

## 📂 Examples
Check out the full runnable examples in the `examples/` directory for more details:
- `01_insert.rs`: Full insertion flow.
- `02_query.rs`: Select patterns and raw queries.
- `03_update.rs`: Updating records.
- `04_delete.rs`: Deletion patterns.
- `05_pagination.rs`: Paging through data.
- `06_transaction.rs`: Working with transactions.
