use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Room {
    pub id: i64,
    pub name: String,
    pub last_message: String,
    pub user_ids: String,
    pub created_at: String,
}

#[derive(Deserialize, Debug)]
pub struct RoomsResponse {
    pub rooms: Vec<Room>,
}
