use sqlx::SqlitePool;

//https://tms-dev-blog.com/rust-sqlx-basics-with-sqlite/
//https://blog.logrocket.com/real-time-chat-app-rust-react/

//cargo install sqlx-cli

//cd fc-server
//sqlx db create --database-url sqlite://fc.db
//sqlx migrate run --database-url sqlite://fc.db

//*** create new migration
//sqlx migrate add users

const DB_URL: &str = "sqlite://fc-server/fc.db";
const DB_URL_TEST: &str = "sqlite://fc.db";
pub async fn db_connection() -> anyhow::Result<SqlitePool> {
    Ok(SqlitePool::connect(DB_URL).await?)
}

pub async fn db_connection_test() -> anyhow::Result<SqlitePool> {
    Ok(SqlitePool::connect(DB_URL_TEST).await?)
}
