use crate::http::model::{MessagesResponse, RoomsResponse};

pub const SERVER_ADDR: &str = "http://localhost:3000";
pub type Result<T> = std::result::Result<T, crate::error::Error>;

pub fn get_user_rooms(user_id: i64) {}

pub fn get_rooms() -> Result<RoomsResponse> {
    let client = reqwest::blocking::Client::new();
    let response = client.get(format!("{}/room", SERVER_ADDR)).send()?;
    dbg!("{}", &response);
    Ok(response.json()?)
}

pub fn get_messages(room_id: i64) -> Result<MessagesResponse> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(format!("{}/message?room_id={}", SERVER_ADDR, room_id))
        .send()?;
    dbg!("{}", &response);
    Ok(response.json()?)
}
