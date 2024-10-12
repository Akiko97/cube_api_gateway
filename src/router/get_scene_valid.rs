use std::sync::Arc;
use axum::{
    Router,
    http::StatusCode,
    response::Response,
    routing::get,
    extract::{State, Path},
};
use crate::message::scene::Scene;
use crate::ServerContext;
use crate::utils::{read_json, to_json};

pub fn setup_routes(router: Router<Arc<ServerContext>>) -> Router<Arc<ServerContext>> {
    router.route("/api/v1/scene/:scene_id/valid", get(handle_get_valid))
}

const JSON_ALICEBOB_INIT: &str = include_str!("../json/AliceBob.init.valid.json");
const JSON_ALICEBOB_TEARDROP: &str = include_str!("../json/AliceBob.teardrop.valid.json");
const JSON_ALICEBOB_BASE64: &str = include_str!("../json/AliceBob.base64.valid.json");
const JSON_ALICEBOB_FILE_CRYPT: &str = include_str!("../json/AliceBob.file_crypt.valid.json");
const JSON_ALICEBOB_FORCE_ATTACK: &str = include_str!("../json/AliceBob.force_attack.valid.json");

/// 获取场景的校验信息
#[utoipa::path(
    get,
    path = "/api/v1/scene/{scene_id}/valid",
    operation_id = "获取场景的校验信息",
    tag = "GET",
    params(
        ("scene_id" = string, Path, description = "场景ID，可以为 init、teardrop、base64、file_crypt 或 force_attack")
    ),
    responses(
        (status = 200, description = "返回场景的校验信息", body = Scene),
        (status = 404, description = "请求不存在"),
        (status = 504, description = "请求超时"),
    )
)]
async fn handle_get_valid(
    State(_state): State<Arc<ServerContext>>,
    Path(scene_id): Path<String>,
) -> Result<Response, StatusCode> {
    let rsp = match scene_id.as_str() {
        "init" =>
            to_json::<Scene>(&read_json::<Scene>(JSON_ALICEBOB_INIT).unwrap()).unwrap(),
        "teardrop" =>
            to_json::<Scene>(&read_json::<Scene>(JSON_ALICEBOB_TEARDROP).unwrap()).unwrap(),
        "base64" =>
            to_json::<Scene>(&read_json::<Scene>(JSON_ALICEBOB_BASE64).unwrap()).unwrap(),
        "file_crypt" =>
            to_json::<Scene>(&read_json::<Scene>(JSON_ALICEBOB_FILE_CRYPT).unwrap()).unwrap(),
        "force_attack" =>
            to_json::<Scene>(&read_json::<Scene>(JSON_ALICEBOB_FORCE_ATTACK).unwrap()).unwrap(),
        _ => return Err(StatusCode::NOT_FOUND),
    };
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(rsp.into())
        .unwrap())
}
