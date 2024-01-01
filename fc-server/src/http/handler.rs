use crate::http::api;
use crate::BoxBody;
use hyper::{Request, Response};
use sqlx::SqlitePool;
use std::sync::Arc;

pub async fn http_handler(
    req: Request<hyper::body::Incoming>,
    pool: Arc<SqlitePool>,
) -> crate::Result<Response<BoxBody>> {
    api::handler(req, pool).await
}
