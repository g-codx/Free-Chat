use crate::db::entity::message::Message;
use crate::db::entity::room::Room;
use crate::db::service;
use crate::http::json::message::MessagesResponse;
use crate::http::json::room::RoomsResponse;
use crate::util::server::{full, missing_parameters, numeric};
use crate::BoxBody;
use hyper::{Request, Response, StatusCode};
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::sync::Arc;
use url::form_urlencoded;

pub async fn get_rooms(pool: Arc<SqlitePool>) -> crate::Result<Response<BoxBody>> {
    let rooms = Room::find_all(&pool).await?;
    let json = serde_json::to_string(&RoomsResponse::from(rooms))?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(full(json))
        .unwrap())
}

pub async fn get_user_rooms(
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

    let user_id = if let Some(n) = params.get("user_id") {
        if let Ok(v) = n.parse::<i64>() {
            v
        } else {
            return numeric();
        }
    } else {
        return missing_parameters();
    };

    let rooms = RoomsResponse::from(service::room::get_user_rooms(user_id, &pool).await?);
    let json = serde_json::to_string(&rooms)?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(full(json))
        .unwrap())
}
