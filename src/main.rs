use anyhow::Result;
use flyer_orm::{Executor, Query, SQLite};
use serde::Serialize;
use tokio::{fs::File, io::AsyncWriteExt};
use std::sync::OnceLock;
use flyer_orm::Entity;

// static CONNECTION: OnceLock<Postgres> = OnceLock::new();
static CONNECTION: OnceLock<SQLite> = OnceLock::new();

pub struct Database;

impl Database {
    pub async fn init() {
        // CONNECTION
        //     .set(Postgres::new("postgresql://postgres:test123@localhost:5111/lucas11776").await)
        //     .ok()
        //     .expect("Database already initialized!");
        CONNECTION
            .set(SQLite::new("database.sqlite").await)
            .ok()
            .expect("Database already initialized!");
    }

    pub fn query<'q>() -> Query<'q, SQLite> {
        let db = CONNECTION
            .get()
            .expect("Database not initialized! Call Database::init().await first.");

        Query::new(db)
    }
}

#[derive(Serialize, Entity, Debug)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub notifications: bool,
    // pub token: String,
}

#[derive(Serialize, Entity, Debug)]
pub struct Prompt {
    pub uuid: String,
    pub question: String,
    pub answer: String,
    pub thinking: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    Database::init().await;

    // let users = Database::query("users")
    //     // .join("password_resets", "users.id", "=", "password_resets.user_id")
    //     // .r#where("email", "=", "thembangubeni04@gmail.com")
    //     // .or_where("email", "=", "themba@gmail.com")
    //     .where_group(|group| {
    //         group
    //             .r#where("email", "=", "thembangubeni04@gmail.com");
    //             // .or_where("email", "=", "themba@gmail.com");
    //     })
    //     // .group_by("password_resets.token")
    //     .get::<User>()
    //     .await
    //     .unwrap();

    // for user in users {
    //     println!("\r\nUser: {:?}\r\n", user);
    // }

    let pagination = Database::query()
        .table("prompts")
        .paginate::<Prompt>(1, 2)
        .await
        .unwrap();

    let mut file = File::create("pagination.json")
        .await
        .unwrap();

    file
        .write_all(serde_json::to_string_pretty(&pagination).unwrap().as_bytes())
        .await
        .unwrap();

    println!("\r\nUSERS: {:?}\r\n", pagination);

    Ok(())
}