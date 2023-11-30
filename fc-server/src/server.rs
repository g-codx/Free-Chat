use crate::db::connection::db_connection;
use crate::{db, BoxBody, PeerMap, Rooms, Sessions, SERVER_ADDR};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;

use crate::util::server::is_upgrade_req;

pub async fn run() {
    let listener = TcpListener::bind(SERVER_ADDR)
        .await
        .expect("Failed to bind");
    log::info!("Listening on {}", SERVER_ADDR);

    let sessions = Sessions::new(RwLock::new(HashMap::new()));
    let poll = Arc::new(db_connection().await.unwrap());
    let rooms = db::service::room::get_all_rooms_map(&poll).await.unwrap();

    loop {
        let (tcp, addr) = listener.accept().await.expect("Failed to accept");
        let io = TokioIo::new(tcp);

        let sessions = sessions.clone();
        let rooms = rooms.clone();
        let pool = poll.clone();

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(
                    io,
                    service_fn(|req: Request<hyper::body::Incoming>| {
                        handle_request(req, addr, sessions.clone(), rooms.clone(), pool.clone())
                    }),
                )
                .with_upgrades()
                .await
            {
                log::error!("Error serving connection: {:?}", err);
            }
        });
    }
}

async fn handle_request(
    req: Request<hyper::body::Incoming>,
    peer_addr: SocketAddr,
    sessions: Sessions,
    rooms: Rooms,
    pool: Arc<SqlitePool>,
) -> crate::Result<Response<BoxBody>> {
    if is_upgrade_req(&req) {
        crate::ws::handler::ws_handler(req, peer_addr, sessions, rooms, pool.clone()).await
    } else {
        crate::http::handler::http_handler(req, pool.clone())
    }
}
