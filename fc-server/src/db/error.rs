#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Room with id `{room_id}` not found")]
    RoomNotFound { room_id: i64 },
    #[error("User `{user_id}` is already in the `{room_id}` room")]
    AlreadyInRoom { user_id: i64, room_id: i64 },
}
