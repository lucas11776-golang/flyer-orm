use anyhow::Result;
use flyer_orm::{Connection, Executor, Query, postgres::Postgres};
use once_cell::sync::OnceCell;
use sqlx::PgPool;
use flyer_orm::Entity;


// pub struct Database {
//     dashboard: Connection<Postgres>,
//     client: Connection<Postgres>,
// }

pub(crate) static mut GLOBAL_SERVER: OnceCell<Box<Postgres>> = OnceCell::new();

pub struct Database {
    inner: Postgres
}

impl Database {
    #[allow(static_mut_refs)]
    pub async fn init() {
        unsafe {
            GLOBAL_SERVER
                .set(Box::new(Postgres::new("postgresql://postgres:test123@localhost:5111/lucas11776").await))
                .map_err(|_| "global state already initialized")
                .unwrap();
        }
    }

    #[allow(static_mut_refs)]
    pub fn query<'q>(table: impl Into<String>) -> Query<'q, Postgres> {
        return unsafe {
            Query::new(GLOBAL_SERVER.get_mut().unwrap().as_mut(), table)
        };
    }
}


#[derive(Entity, Debug)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub notifications: bool,
}


#[tokio::main]
async fn main() -> Result<()> {
    Database::init().await;

    let users = Database::query("users")
        .r#where("email", "=", "thembangubeni04@gmail.com")
        .get::<User>()
        .await
        .unwrap();

    for user in users {
        println!("\r\n\r\n\r\nUser:{:?}\r\n\r\n\r\n", user);
    }

    Ok(())
}