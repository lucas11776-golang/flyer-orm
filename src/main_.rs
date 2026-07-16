use anyhow::Result;
use flyer_orm::Query;
use sqlx::PgPool;
use flyer_orm::Entity;

#[derive(Entity, Debug)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub notifications: bool,
}

pub async fn all() -> Result<Vec<User>> {
    let pool = PgPool::connect("postgresql://postgres:test123@localhost:5111/lucas11776")
        .await
        .unwrap();

    return Query::<sqlx::Postgres>::new("users")
        .get(&pool)
        .await
        .map_err(|err| err.into());
}

#[tokio::main]
 async fn main() -> Result<()> {
    let users = all()
        .await
        .unwrap();

    for user in users {
        println!("\r\n\r\n\r\nFound User: {:?}\r\n\r\n\r\n", user);
    }

    Ok(())
}