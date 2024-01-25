use crate::util::server::not_found;
use crate::BoxBody;
use hyper::{Method, Request, Response};
use sqlx::SqlitePool;
use std::sync::Arc;

mod message;
mod room;

pub async fn handler(
    req: Request<hyper::body::Incoming>,
    pool: Arc<SqlitePool>,
) -> crate::Result<Response<BoxBody>> {
    dbg!("req uri {}", req.uri().path());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/room") => room::get::get_rooms(pool.clone()).await,
        (&Method::GET, "/user-room") => room::get::get_user_rooms(&req, pool.clone()).await,
        (&Method::GET, "/message") => message::get::get_messages(&req, pool.clone()).await,
        _ => not_found(),
    }
}
