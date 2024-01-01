use crate::db::entity::user::User;
use crate::db::service::message::add_message;
use crate::db::service::room as room_service;
use crate::session::Session;
use crate::util::server::full;
use crate::{BoxBody, Rooms, Sessions, UserIds};
use fc_command::Command;
use futures_util::{FutureExt, StreamExt};
use hyper::http::HeaderValue;
use hyper::upgrade::Upgraded;
use hyper::{header, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use sqlx::SqlitePool;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{
    tungstenite::{handshake::derive_accept_key, protocol::Role},
    WebSocketStream,
};

pub async fn ws_handler(
    req: Request<hyper::body::Incoming>,
    peer_addr: SocketAddr,
    sessions: Sessions,
    rooms: Rooms,
    pool: Arc<SqlitePool>,
) -> crate::Result<Response<BoxBody>> {
    let path = req.uri().path();

    //it has to be jwt
    let user_id = match path.chars().last().filter(|l| l.is_numeric()) {
        None => {
            log::info!("HTTP 400 Bad request");
            let mut response = Response::new(full("HTTP 400 Bad request"));
            *response.status_mut() = StatusCode::BAD_REQUEST;
            return Ok(response);
        }
        Some(user_id) => user_id.to_digit(10).unwrap() as i64,
    };

    match User::find_by_id(user_id, &pool).await {
        Ok(some_user) => match some_user {
            None => {
                log::info!("HTTP 401 Unauthorized");
                let mut response = Response::new(full("HTTP 401 Unauthorized"));
                *response.status_mut() = StatusCode::UNAUTHORIZED;
                return Ok(response);
            }
            Some(_) => {
                log::info!("Authorization successful: {}", user_id);
            }
        },
        Err(err) => {
            log::info!("{}", err);
            let mut response = Response::new(full("HTTP 500 Internal server error"));
            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            return Ok(response);
        }
    }

    let accept_key = accept_key(&req);
    tokio::task::spawn(async move {
        match hyper::upgrade::on(req).await {
            Ok(upgraded) => {
                handle_upgrade(upgraded, peer_addr, sessions, rooms, pool, user_id).await
            }
            Err(err) => log::error!("Upgrade error: {}", err),
        }
    });

    upgrade_response(accept_key)
}

async fn handle_upgrade(
    upgraded: Upgraded,
    peer_addr: SocketAddr,
    sessions: Sessions,
    rooms: Rooms,
    pool: Arc<SqlitePool>,
    user_id: i64,
) {
    let upgraded = TokioIo::new(upgraded);
    let ws_stream = WebSocketStream::from_raw_socket(upgraded, Role::Server, None).await;
    log::info!("WebSocket connection established: {}", peer_addr);

    let (ws_sender, mut ws_rcv) = ws_stream.split();
    let (sender, rcv) = mpsc::unbounded_channel();
    let rcv = UnboundedReceiverStream::new(rcv);

    tokio::task::spawn(rcv.forward(ws_sender).map(|result| {
        if let Err(err) = result {
            log::error!("Error sending websocket msg: {}", err);
        }
    }));

    let session = Session::new(peer_addr, user_id, sender);
    sessions.write().await.insert(user_id, session);
    log::info!("{} connected", peer_addr);

    while let Some(result) = ws_rcv.next().await {
        let ws_message = match result {
            Ok(msg) => msg,
            Err(err) => {
                log::error!("Error receiving ws message for: {}; {}", peer_addr, err);
                break;
            }
        };

        message_handler(ws_message, user_id, &sessions, &rooms, &pool).await;
    }

    sessions.write().await.remove(&user_id);
    log::info!("{} disconnected", peer_addr);
}

async fn message_handler(
    msg: Message,
    user_id: i64,
    sessions: &Sessions,
    rooms: &Rooms,
    pool: &Arc<SqlitePool>,
) {
    log::info!("Received message from {}: {:?}", user_id, msg);
    match msg {
        Message::Binary(bytes) => handle_message(bytes, user_id, sessions, rooms, pool).await,
        _ => log::error!("Unsupported message type"),
    }
}

async fn handle_message(
    bytes: Vec<u8>,
    user_id: i64,
    sessions: &Sessions,
    rooms: &Rooms,
    pool: &Arc<SqlitePool>,
) {
    let read_lock = sessions.read().await;
    let session = read_lock.get(&user_id).unwrap();

    let command = match fc_command::decode(bytes) {
        Ok(cmd) => cmd,
        Err(err) => {
            log::error!("{}", err);
            session
                .tx
                .send(Ok(Message::Binary(b"Unknown commnd".to_vec())))
                .unwrap();
            return;
        }
    };

    match command {
        Command::CreateRoom(room_name) => {
            match room_service::create_room(room_name, user_id, pool).await {
                Ok(id) => {
                    rooms.write().await.insert(id, UserIds::from([user_id]));
                    session
                        .tx
                        .send(Ok(Message::Binary(b"OK".to_vec())))
                        .unwrap();
                }
                Err(err) => {
                    log::error!("{}", err);
                    session
                        .tx
                        .send(Ok(Message::Binary(b"Can not create room".to_vec())))
                        .unwrap();
                }
            }
        }
        Command::JoinRoom(room_id) => {
            match room_service::add_user_to_room(room_id, user_id, pool).await {
                Ok(_) => {
                    let mut lock = rooms.write().await;
                    if let Some(room) = lock.get_mut(&room_id) {
                        room.insert(user_id);

                        session
                            .tx
                            .send(Ok(Message::Binary(b"OK".to_vec())))
                            .unwrap();
                    } else {
                        //?
                        log::error!("database and in-memory storage are not synchronized");
                    }
                }
                Err(err) => {
                    log::error!("{}", err);
                    session
                        .tx
                        .send(Ok(Message::Binary(b"Failed to join the room".to_vec())))
                        .unwrap();
                }
            }
        }
        Command::LeaveRoom(room_id) => {
            match room_service::remove_user_from_room(room_id, user_id, pool).await {
                Ok(_) => {
                    let mut lock = rooms.write().await;
                    if let Some(room) = lock.get_mut(&room_id) {
                        room.remove(&user_id);

                        session
                            .tx
                            .send(Ok(Message::Binary(b"OK".to_vec())))
                            .unwrap();
                    } else {
                        //?
                        log::error!("database and in-memory storage are not synchronized");
                    }
                }
                Err(err) => {
                    log::error!("{}", err);
                    session
                        .tx
                        .send(Ok(Message::Binary(b"Failed to leave the room".to_vec())))
                        .unwrap();
                }
            }
        }
        Command::SendMessage(room_id, message) => {
            let room_lock = rooms.read().await;
            let sessions_lock = sessions.read().await;
            if let Some(room_users) = room_lock.get(&room_id) {
                match add_message(user_id, room_id, message.clone(), pool).await {
                    Ok(_) => {
                        for room_user_id in room_users {
                            if user_id != *room_user_id {
                                if let Some(session) = sessions_lock.get(room_user_id) {
                                    session
                                        .tx
                                        .send(Ok(Message::Binary(message.clone())))
                                        .unwrap();
                                }
                            }
                        }
                    }
                    Err(err) => {
                        log::error!("{}", err);
                    }
                }
            }
        }
    }
}

fn upgrade_response(accept_key: HeaderValue) -> crate::Result<Response<BoxBody>> {
    let mut response = Response::new(full("HTTP 101 Switching Protocols"));
    *response.status_mut() = StatusCode::SWITCHING_PROTOCOLS;
    response
        .headers_mut()
        .insert(header::UPGRADE, HeaderValue::from_static("websocket"));
    response
        .headers_mut()
        .insert(header::CONNECTION, HeaderValue::from_static("Upgrade"));
    response
        .headers_mut()
        .insert(header::SEC_WEBSOCKET_ACCEPT, accept_key);

    Ok(response)
}

fn accept_key(req: &Request<hyper::body::Incoming>) -> HeaderValue {
    req.headers()
        .get(header::SEC_WEBSOCKET_KEY)
        .map(|sk| derive_accept_key(sk.as_bytes()))
        .map(|ak| ak.parse().unwrap_or(HeaderValue::from_static("")))
        .unwrap_or(HeaderValue::from_static(""))
}
