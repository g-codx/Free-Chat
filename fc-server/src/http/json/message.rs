use crate::db::entity::message::Message;
use serde::Serialize;

#[derive(Serialize)]
pub struct MessagesResponse {
    messages: Vec<Message>,
}

impl From<Vec<Message>> for MessagesResponse {
    fn from(value: Vec<Message>) -> Self {
        Self { messages: value }
    }
}
