use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
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

#[derive(Deserialize, Debug)]
pub struct Message {
    pub id: i64,
    pub user_id: i64,
    pub room_id: i64,
    content: Vec<u8>,
    pub created_at: String,
}

impl Message {
    pub fn get_content(&self) -> String {
        String::from_utf8(self.content.clone()).unwrap_or_default()
    }
}

#[derive(Deserialize, Debug)]
pub struct MessagesResponse {
    pub messages: Vec<Message>,
}
