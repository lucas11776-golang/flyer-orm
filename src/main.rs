use anyhow::Result;
use flyer_orm::{DB, Executor, query::Order, sqlite::SQLite};
use serde::Serialize;
use sqlx::{Any, AnyPool, Column, FromRow, Pool, Row, SqlitePool, any::install_default_drivers, query::QueryAs, sqlite::SqliteRow};

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct User {
    pub uuid: String,
    pub created_at: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}


#[allow(async_fn_in_trait)]
pub trait Database {
    type T: sqlx::Database;

    async fn get<O>(&self) -> Result<Vec<O>>
    where
        O: for<'r> FromRow<'r,  <Self::T as sqlx::Database>::Row> + Send + Unpin
        ;

}

pub struct SQlite {
    db: Pool<sqlx::Sqlite>
}

impl SQlite {
    pub async fn new(url: &str) -> Self {
        Self {
            db: SqlitePool::connect(url).await.unwrap()
        }
    }
}

impl Database for SQlite {
    type T = sqlx::Sqlite;
    
    async fn get<O>(&self) -> Result<Vec<O>>
    where
        O: for<'r> FromRow<'r, <Self::T as sqlx::Database>::Row> + Send + Unpin
    {     
        return Ok(
            sqlx::query_as::<Self::T, O>("SELECT * FROM users")
                .fetch_all(&self.db)
                .await
                .unwrap()
        )
    }
    
    
}

#[tokio::main]
async fn main() -> Result<()> {
    // TODO: find way to make data dynamic -> `DB::query("sqlite").table("users")`
    let users = SQLite::connect("./database.sqlite").await
        .table("users")
        .select(vec!["*"])
        .first::<User>()
        .await
        .unwrap();
        
        





















    // DB::add("sqlite", SQLite::connect("./database.sqlite").await.unwrap());

    // let users = DB::query("sqlite")
    //     .table("users")
    //     .limit(10)
    //     .order_by("created_at", Order::DESC)
    //     .get::<User>(20)
    //     .await
    //     .unwrap();

    // install_default_drivers();



    // let db = SQlite::new("./database.sqlite").await;

    // let users = db.get::<User>().await.unwrap();


    // println!("{:?}", users);

    // let sqlite_pool = SqlitePool::connect("sqlite:./database.sqlite").await?;


    // let users: Vec<User> =
    //     sqlx::query_as::<sqlx::Sqlite, User>("SELECT * FROM users")
    //         .fetch_all(&sqlite_pool).await?;


    // let a = sqlx::query_as(sql)

    // println!("{:?}", users);

    Ok(())
}