use crate::db::entity::room::Room;
use crate::http::json::room::RoomsResponse;
use crate::util::server::full;
use crate::BoxBody;
use hyper::{Response, StatusCode};
use sqlx::SqlitePool;
use std::sync::Arc;

pub async fn get_rooms(pool: Arc<SqlitePool>) -> crate::Result<Response<BoxBody>> {
    let rooms = Room::find_all(&pool).await?;
    let json = serde_json::to_string(&RoomsResponse::from(rooms))?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(full(json))
        .unwrap())
}
