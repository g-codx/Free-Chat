use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

#[derive(Serialize, FromRow, Clone, Debug)]
pub struct Room {
    pub id: i64,
    pub name: String,
    pub last_message: String,
    pub user_ids: String,
    pub created_at: String,
}

impl Room {
    pub async fn insert(name: String, user_ids: String, pool: &SqlitePool) -> anyhow::Result<i64> {
        let id = sqlx::query(
            "insert into room ( name, last_message, user_ids, created_at ) values (?, ?, ?, ?)",
        )
        .bind(name)
        .bind(String::new())
        .bind(user_ids)
        .bind(String::new())
        .execute(pool)
        .await?
        .last_insert_rowid();

        Ok(id)
    }

    pub async fn find_by_id(id: i64, pool: &SqlitePool) -> anyhow::Result<Option<Room>> {
        let room = sqlx::query_as::<_, Room>(
            "select id, name, last_message, user_ids, created_at from room where id = ?",
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(room)
    }

    pub async fn update_user_ids_by_id(
        id: i64,
        user_ids: String,
        pool: &SqlitePool,
    ) -> anyhow::Result<()> {
        sqlx::query("update room set user_ids = ? where id = ?")
            .bind(user_ids)
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn find_all(pool: &SqlitePool) -> anyhow::Result<Vec<Room>> {
        let rooms = sqlx::query_as("select id, name, last_message, user_ids, created_at from room")
            .fetch_all(pool)
            .await?;

        Ok(rooms)
    }
}
