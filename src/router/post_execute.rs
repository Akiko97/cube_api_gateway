use std::sync::Arc;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Response;
use axum::{Json, Router};
use axum::routing::post;
use crate::ServerContext;

pub fn setup_routes(router: Router<Arc<ServerContext>>) -> Router<Arc<ServerContext>> {
    router.route("/api/v1/scene/:scene_id/execute", post(handle_execute))
}

/// 执行场景
#[utoipa::path(
    post,
    path = "/api/v1/scene/{scene_id}/execute",
    operation_id = "执行场景",
    tag = "POST",
    params(
        ("scene_id" = string, Path, description = "场景ID，可以为 init、teardrop、base64、file_crypt 或 force_attack")
    ),
    request_body(content = Scene, description = "当前场景的场景数据"),
    responses(
        (status = 200, description = "返回正确的场景信息", body = Scene),
        (status = 404, description = "请求不存在"),
        (status = 504, description = "请求超时"),
        (status = 422, description = "无法解析请求的JSON"),
    )
)]
async fn handle_execute(
    State(_state): State<Arc<ServerContext>>,
    Path(_scene_id): Path<String>,
    Json(_payload): Json<crate::message::scene::Scene>,
) -> Result<Response, StatusCode> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body("{}".into())
        .unwrap())
}
