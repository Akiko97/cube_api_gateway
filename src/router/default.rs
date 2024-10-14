use std::sync::Arc;
use axum::{
    Router,
    response::Response,
    http::StatusCode,
    extract::Request
};
use crate::ServerContext;

pub fn setup_routes(router: Router<Arc<ServerContext>>) -> Router<Arc<ServerContext>> {
    router.fallback(handle_default)
}

const DEFAULT_PAGE: &str = include_str!("default.html");

async fn handle_default(
    _req: Request,
) -> Result<Response, StatusCode> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(DEFAULT_PAGE.into())
        .unwrap())
}
