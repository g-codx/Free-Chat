use crate::db::entity::message::Message;
use crate::util::server::{full, internal_server_error, missing_parameters, not_found, numeric};
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

    match db::service::message::get_messages(room_id, &pool).await {
        Ok(resposne) => {
            todo!()
        }
        Err(err) => {
            log::error!("{}", err);
            internal_server_error()
        }
    }
}
