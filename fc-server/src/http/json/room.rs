use serde::{Serialize};
use crate::db::entity::room::Room;

#[derive(Serialize)]
pub struct RoomsResponse {
    pub rooms: Vec<Room>
}

impl From<Vec<Room>> for RoomsResponse {
    fn from(value: Vec<Room>) -> Self {
        Self {rooms: value}
    }
}