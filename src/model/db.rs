// use sqlx::postgres::PgPoolOptions;
// use sqlx::{Pool, Postgres};
// use std::error::Error;
// use std::time::Duration;
//
// pub type Db = Pool<Postgres>;
//
// pub fn init_db() -> Result<Db, sqlx::Error>{
//
// }
//
// async fn new_db_pool(
//     host: &str,
//     port: &u8,
//     user: &str,
//     password: &str,
//     db_name: &str,
//     max_conn: &u32
// ) -> Result<Db, sqlx::Error> {
//     let db_url = format!("postgres://{}:{}@{}:{}/{}", user, password, host, port, db_name);
//     PgPoolOptions::new()
//         .max_connections(*max_conn)
//         .connect_timeout(Duration::from_millis(500))  // 500 ms connection timeout
//         .connect(&db_url)
//         .await
// }
