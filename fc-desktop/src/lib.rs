pub mod app;
pub mod core;
mod error;
pub mod http;
pub mod ws;

pub type WsMessage = tokio_tungstenite::tungstenite::Message;
