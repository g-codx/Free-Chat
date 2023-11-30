use crate::util::server::full;
use crate::BoxBody;
use hyper::{Request, Response};
use sqlx::SqlitePool;
use std::sync::Arc;

pub fn http_handler(
    _req: Request<hyper::body::Incoming>,
    pool: Arc<SqlitePool>,
) -> crate::Result<Response<BoxBody>> {
    Ok(Response::new(full("Hello world")))
}
