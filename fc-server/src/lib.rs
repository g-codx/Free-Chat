use crate::db::entity::room::Room;
use crate::session::Session;
use bytes::Bytes;
use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tokio_tungstenite::tungstenite::Message;

pub mod db;
mod http;
pub mod server;
mod session;
mod util;
mod ws;

pub const SERVER_ADDR: &str = "127.0.0.1:3000";

pub type GenericError = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, GenericError>;
pub type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;
pub type Tx =
    mpsc::UnboundedSender<core::result::Result<Message, tokio_tungstenite::tungstenite::Error>>;
pub type PeerMap = Arc<RwLock<HashMap<SocketAddr, Tx>>>;
pub type UserIds = HashSet<i64>;
pub type Sessions = Arc<RwLock<HashMap<i64, Session>>>;
pub type Rooms = Arc<RwLock<HashMap<i64, UserIds>>>;

// pub type Sessions = Arc<RwLock<HashMap<i64, Arc<Session>>>>;
// pub type Rooms = Arc<RwLock<HashMap<i64, Vec<Arc<Session>>>>>;
