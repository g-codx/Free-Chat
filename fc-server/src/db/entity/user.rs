use sqlx::{FromRow, SqlitePool};

#[derive(FromRow, Clone, Debug)]
pub struct User {
    id: i64,
    name: String,
    password: String,
    created_at: String,
}

impl User {
    pub async fn insert(name: String, password: String, pool: &SqlitePool) -> anyhow::Result<i64> {
        let id = sqlx::query("insert into user (name, password, created_at) values(?,?,?)")
            .bind(name)
            .bind(password)
            .bind("01.01.1900")
            .execute(pool)
            .await?
            .last_insert_rowid();

        Ok(id)
    }

    pub async fn find_by_id(id: i64, pool: &SqlitePool) -> anyhow::Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "select id, name, password, created_at from user where id = ?",
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }
}
