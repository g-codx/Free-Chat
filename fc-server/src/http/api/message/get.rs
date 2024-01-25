use crate::db::entity::message::Message;
use crate::http::json::message::MessagesResponse;
use crate::util::server::{full, missing_parameters, numeric};
use crate::{db, BoxBody};
use hyper::{Request, Response, StatusCode};
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::sync::Arc;
use url::form_urlencoded;

pub async fn get_messages(
    req: &Request<hyper::body::Incoming>,
    pool: Arc<SqlitePool>,
) -> crate::Result<Response<BoxBody>> {
    let query = if let Some(q) = req.uri().query() {
        q
    } else {
        return missing_parameters();
    };

    let params = form_urlencoded::parse(query.as_bytes())
        .into_owned()
        .collect::<HashMap<String, String>>();

    let room_id = if let Some(n) = params.get("room_id") {
        if let Ok(v) = n.parse::<i64>() {
            v
        } else {
            return numeric();
        }
    } else {
        return missing_parameters();
    };

    let messages = MessagesResponse::from(Message::find_by_room_id(room_id, &pool).await?);
    let json = serde_json::to_string(&messages)?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(full(json))
        .unwrap())
}
