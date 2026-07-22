use anyhow::Result;
use flyer_orm::{Executor, Postgres, Query};
use std::sync::OnceLock;
use flyer_orm::Entity;

static CONNECTION: OnceLock<Postgres> = OnceLock::new();

pub struct Database;

impl Database {
    pub async fn init() {
        CONNECTION
            .set(Postgres::new("postgresql://postgres:test123@localhost:5111/lucas11776").await)
            .ok()
            .expect("Database already initialized!");
    }

    pub fn query<'q>(table: impl Into<String>) -> Query<'q, Postgres> {
        let db = CONNECTION
            .get()
            .expect("Database not initialized! Call Database::init().await first.");

        Query::new(db, table.into())
    }
}

#[derive(Entity, Debug)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub notifications: bool,
    // pub token: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    Database::init().await;

    let users = Database::query("users")
        // .join("password_resets", "users.id", "=", "password_resets.user_id")
        // .r#where("email", "=", "thembangubeni04@gmail.com")
        // .or_where("email", "=", "themba@gmail.com")
        .where_group(|group| {
            group
                .r#where("email", "=", "thembangubeni04@gmail.com");
                // .or_where("email", "=", "themba@gmail.com");
        })
        // .group_by("password_resets.token")
        .get::<User>()
        .await
        .unwrap();

    for user in users {
        println!("\r\nUser: {:?}\r\n", user);
    }

    Ok(())
}