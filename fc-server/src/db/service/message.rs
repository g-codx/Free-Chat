use crate::db::entity::message::Message;
use sqlx::SqlitePool;

pub async fn add_message(
    user_id: i64,
    room_id: i64,
    content: Vec<u8>,
    pool: &SqlitePool,
) -> anyhow::Result<()> {
    Message::insert(user_id, room_id, content, pool).await?;
    Ok(())
}

pub async fn get_messages(room_id: i64, pool: &SqlitePool) -> anyhow::Result<Vec<Message>> {
    Message::find_by_room_id(room_id, pool).await
}
