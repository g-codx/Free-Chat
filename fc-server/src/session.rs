use crate::Tx;
use std::net::SocketAddr;

#[derive(Debug)]
pub struct Session {
    pub socket_addr: SocketAddr,
    pub user_id: i64,
    pub tx: Tx,
}

impl Session {
    pub fn new(socket_addr: SocketAddr, user_id: i64, tx: Tx) -> Self {
        Self {
            socket_addr,
            user_id,
            tx,
        }
    }
}
