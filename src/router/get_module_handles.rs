use std::sync::Arc;
use axum::{
    Router,
    routing::get,
    response::Response,
    http::StatusCode,
    extract::{Path, State}
};
use crate::ServerContext;

pub fn setup_routes(router: Router<Arc<ServerContext>>) -> Router<Arc<ServerContext>> {
    router.route("/api/v1/scene/:scene_id/module_handles", get(handle_get_module_handles))
}

const JSON_ALICEBOB_INIT: &str = include_str!("../json/AliceBob.init.handles.json");
const JSON_ALICEBOB_TEARDROP: &str = include_str!("../json/AliceBob.teardrop.handles.json");
const JSON_ALICEBOB_BASE64: &str = include_str!("../json/AliceBob.base64.handles.json");
const JSON_ALICEBOB_FILE_CRYPT: &str = include_str!("../json/AliceBob.file_crypt.handles.json");
const JSON_ALICEBOB_FORCE_ATTACK: &str = include_str!("../json/AliceBob.force_attack.handles.json");

/// 获取模块的操作
#[utoipa::path(
    get,
    path = "/api/v1/scene/{scene_id}/module_handles",
    operation_id = "获取模块的操作",
    tag = "GET",
    params(
        ("scene_id" = string, Path, description = "场景ID，可以为 init、teardrop、base64、file_crypt 或 force_attack")
    ),
    responses(
        (status = 200, description = "返回场景中模块可以执行的操作", body = [ModuleHandles]),
        (status = 404, description = "请求不存在"),
        (status = 504, description = "请求超时"),
    )
)]
async fn handle_get_module_handles(
    State(_state): State<Arc<ServerContext>>,
    Path(scene_id): Path<String>,
) -> Result<Response, StatusCode> {
    let rsp = match scene_id.as_str() {
        "init" => JSON_ALICEBOB_INIT,
        "teardrop" => JSON_ALICEBOB_TEARDROP,
        "base64" => JSON_ALICEBOB_BASE64,
        "file_crypt" => JSON_ALICEBOB_FILE_CRYPT,
        "force_attack" => JSON_ALICEBOB_FORCE_ATTACK,
        _ => return Err(StatusCode::NOT_FOUND),
    };
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(rsp.into())
        .unwrap())
}
