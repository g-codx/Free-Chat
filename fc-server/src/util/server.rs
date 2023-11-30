use crate::BoxBody;
use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper::{header, Request};

pub fn is_upgrade_req(req: &Request<hyper::body::Incoming>) -> bool {
    req.headers().contains_key(header::UPGRADE)
}

pub fn full<T: Into<Bytes>>(chunk: T) -> BoxBody {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}
