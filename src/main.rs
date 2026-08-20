use anyhow::Result;
use flyer_orm::{Database, Entity, Executor, Postgres, SQLite};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Entity)]
pub struct Projects {
    pub uuid: String,
    pub organization_uuid: String,
    pub name: String,
    pub description: String,
    pub prompts: i64,
}







pub fn db<'q>() -> Database<'q, Postgres> {
    Database::<Postgres>::connection("default")
}

pub async fn destroy(id: String) -> Result<()> {
    db()
        .delete("users")
        .r#where("id", "=", id)
        .execute()
        .await
}

#[tokio::main]
async fn main() -> Result<()> {
    // Database::add("default", SQLite::new("database.sqlite").await);
    Database::add("default", Postgres::new("postgresql://postgres:test123@localhost:5111/lucas11776").await);

    // let project = db()
    //     .raw_read("sql/projects.sql")
    //     .bind(String::from("296598c0-095c-4c88-a48c-8af6c98022ff"))
    //     .first::<Projects>()
    //     .await
    //     .unwrap();

    // println!("PROJECT -> {:?}", project);

    // #[derive(Debug, Entity)]
    // pub struct User {
    //     pub first_name: String,
    //     pub last_name: String,
    //     pub email: String,
    // }



    // #[derive(Debug, Entity)]
    // pub struct UserEntity {
    //     pub first_name: String,
    //     pub last_name: String,
    //     pub email: String,
    // }

    // // 1. Multi-column query into UserEntity
    // let user: UserEntity = db()
    //     .scaler("SELECT first_name, last_name, email FROM users")
    //     .first()
    //     .await
    //     .unwrap();


    #[derive(Debug, Entity, Serialize, Deserialize)]
    pub struct UserJson {
        pub first_name: String,
        pub last_name: String,
        pub email: String,
    }

    let user: sqlx::types::Json<UserJson> = db()
        .scaler("
            SELECT jsonb_build_object(
                'first_name', users.first_name,
                'last_name', users.last_name,
                'email', users.email
            )
            FROM users
        ")
        .first()
        .await
        .unwrap();

        

    println!("PROJECT -> {:?}", user);
        

    
    Ok(())
}