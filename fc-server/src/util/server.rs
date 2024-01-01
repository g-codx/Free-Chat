use crate::BoxBody;
use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper::{header, Request, Response, StatusCode};

const NOT_FOUND: &[u8] = b"Not Found";
const MISSING_PARAMETERS: &[u8] = b"Missing parameters";
const NUMERIC: &[u8] = b"Number parameter is not numeric";
const INTERNAL_SERVER_ERROR: &[u8] = b"Internal Server Error";

pub fn is_upgrade_req(req: &Request<hyper::body::Incoming>) -> bool {
    req.headers().contains_key(header::UPGRADE)
}

pub fn full<T: Into<Bytes>>(chunk: T) -> BoxBody {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

pub fn not_found() -> crate::Result<Response<BoxBody>> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(full(NOT_FOUND))
        .unwrap())
}

pub fn internal_server_error() -> crate::Result<Response<BoxBody>> {
    Ok(Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(full(INTERNAL_SERVER_ERROR))
        .unwrap())
}

pub fn missing_parameters() -> crate::Result<Response<BoxBody>> {
    Ok(Response::builder()
        .status(StatusCode::UNPROCESSABLE_ENTITY)
        .body(full(MISSING_PARAMETERS))
        .unwrap())
}

pub fn numeric() -> crate::Result<Response<BoxBody>> {
    Ok(Response::builder()
        .status(StatusCode::UNPROCESSABLE_ENTITY)
        .body(full(NUMERIC))
        .unwrap())
}
