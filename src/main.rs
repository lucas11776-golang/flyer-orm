use anyhow::Result;
use flyer_orm::{Query, postgres::Postgres};
use once_cell::sync::OnceCell;
use flyer_orm::Entity;

pub(crate) static mut GLOBAL_SERVER: OnceCell<Box<Postgres>> = OnceCell::new();

pub struct Database {
    // inner: Postgres
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
                .r#where("email", "=", "thembangubeni04@gmail.com")
                .or_where("email", "=", "themba@gmail.com");
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