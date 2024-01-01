use crate::http::model::RoomsResponse;

pub const SERVER_ADDR: &str = "http://localhost:3000";
pub type Result<T> = std::result::Result<T, crate::error::Error>;

pub fn get_rooms() -> Result<RoomsResponse> {
    let client = reqwest::blocking::Client::new();
    let response = client.get(format!("{}/room", SERVER_ADDR)).send()?;
    dbg!("{}", &response);
    Ok(response.json()?)
}
