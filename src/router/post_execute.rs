use std::sync::Arc;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Response;
use axum::{Json, Router};
use axum::routing::post;
use crate::message::execute::ExecuteRsp;
use crate::ServerContext;
use crate::utils::{read_json, to_json};

pub fn setup_routes(router: Router<Arc<ServerContext>>) -> Router<Arc<ServerContext>> {
    router.route("/api/v1/scene/:scene_id/execute", post(handle_execute))
}

const JSON_ALICEBOB_INIT: &str = include_str!("../json/AliceBob.init.execute.json");
const JSON_ALICEBOB_TEARDROP: &str = include_str!("../json/AliceBob.teardrop.execute.json");
const JSON_ALICEBOB_BASE64: &str = include_str!("../json/AliceBob.base64.execute.json");
const JSON_ALICEBOB_FILE_CRYPT: &str = include_str!("../json/AliceBob.file_crypt.execute.json");
const JSON_ALICEBOB_FORCE_ATTACK: &str = include_str!("../json/AliceBob.force_attack.execute.json");

/// 执行场景
#[utoipa::path(
    post,
    path = "/api/v1/scene/{scene_id}/execute",
    operation_id = "执行场景",
    tag = "POST",
    params(
        ("scene_id" = string, Path, description = "场景ID，可以为 init、teardrop、base64、file_crypt 或 force_attack")
    ),
    request_body(content = ExecuteReq, description = "当前场景的场景数据"),
    responses(
        (status = 200, description = "返回执行的 Log", body = ExecuteRsp),
        (status = 404, description = "请求不存在"),
        (status = 504, description = "请求超时"),
        (status = 422, description = "无法解析请求的JSON"),
    )
)]
async fn handle_execute(
    State(_state): State<Arc<ServerContext>>,
    Path(scene_id): Path<String>,
    Json(_payload): Json<crate::message::execute::ExecuteReq>,
) -> Result<Response, StatusCode> {
    let rsp = match scene_id.as_str() {
        "init" =>
            to_json::<ExecuteRsp>(&read_json::<ExecuteRsp>(JSON_ALICEBOB_INIT).unwrap()).unwrap(),
        "teardrop" =>
            to_json::<ExecuteRsp>(&read_json::<ExecuteRsp>(JSON_ALICEBOB_TEARDROP).unwrap()).unwrap(),
        "base64" =>
            to_json::<ExecuteRsp>(&read_json::<ExecuteRsp>(JSON_ALICEBOB_BASE64).unwrap()).unwrap(),
        "file_crypt" =>
            to_json::<ExecuteRsp>(&read_json::<ExecuteRsp>(JSON_ALICEBOB_FILE_CRYPT).unwrap()).unwrap(),
        "force_attack" =>
            to_json::<ExecuteRsp>(&read_json::<ExecuteRsp>(JSON_ALICEBOB_FORCE_ATTACK).unwrap()).unwrap(),
        _ => return Err(StatusCode::NOT_FOUND),
    };
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(rsp.into())
        .unwrap())
}
