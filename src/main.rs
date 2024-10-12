mod message;

use axum::{
    routing::post,
    routing::get,
    Router,
    response::Response,
    extract::{ Request, Json },
    http::StatusCode
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(get_scene),
    components(
        schemas(message::scenes::Scene),
        schemas(message::scenes::Edge),
        schemas(message::scenes::Route),
        schemas(message::scenes::Props),
        schemas(message::scenes::Node),
        schemas(message::scenes::NodeType),
        schemas(message::scenes::Module),
        schemas(message::scenes::ModuleType),
        schemas(message::require::GetScene),
    )
)]
struct ApiDoc;

const JSON_ALICEBOB_INIT: &str = include_str!("json/AliceBob.init.json");
const JSON_ALICEBOB_TEARDROP: &str = include_str!("json/AliceBob.teardrop.json");
const JSON_ALICEBOB_BASE64: &str = include_str!("json/AliceBob.base64.json");
const JSON_ALICEBOB_FILE_CRYPT: &str = include_str!("json/AliceBob.file_crypt.json");
const JSON_ALICEBOB_FORCE_ATTACK: &str = include_str!("json/AliceBob.force_attack.json");

#[utoipa::path(
    post,
    path = "/api/v1/scene/get",
    request_body(content = GetScene, description = "获取场景信息的请求"),
    responses(
        (status = 200, description = "返回正确的场景信息", body = Scene),
        (status = 404, description = "请求不存在的场景")
    )
)]
async fn get_scene(
    Json(payload): Json<message::require::GetScene>,
) -> Result<Response, StatusCode> {
    let json = match payload.scene_id {
        0 => JSON_ALICEBOB_INIT,
        1 => JSON_ALICEBOB_TEARDROP,
        2 => JSON_ALICEBOB_BASE64,
        3 => JSON_ALICEBOB_FILE_CRYPT,
        4 => JSON_ALICEBOB_FORCE_ATTACK,
        _ => return Err(StatusCode::NOT_FOUND),
    };
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(json.into())
        .unwrap())
}

const DEFAULT_PAGE: &str = include_str!("default.html");
async fn default(
    _req: Request,
) -> Result<Response, StatusCode> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(DEFAULT_PAGE.into())
        .unwrap())
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/api/v1/scene/get", post(get_scene))
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .fallback(get(default));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
