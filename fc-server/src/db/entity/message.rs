use sqlx::{FromRow, SqlitePool};

#[derive(FromRow, Clone, Debug)]
pub struct Message {
    id: i64,
    user_id: i64,
    room_id: i64,
    content: Vec<u8>,
    created_at: String,
}

impl Message {
    pub async fn insert(
        user_id: i64,
        room_id: i64,
        content: Vec<u8>,
        pool: &SqlitePool,
    ) -> anyhow::Result<i64> {
        let id = sqlx::query(
            "insert into message (user_id, room_id, content, created_at) values (?,?,?,?)",
        )
        .bind(user_id)
        .bind(room_id)
        .bind(content)
        .bind(String::new())
        .execute(pool)
        .await?
        .last_insert_rowid();

        Ok(id)
    }
}
