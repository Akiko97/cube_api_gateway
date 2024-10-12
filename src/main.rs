mod message;
mod router;
mod utils;

use std::sync::Arc;
use axum::Router;

pub struct ServerContext {/* Nothing */}

#[tokio::main]
async fn main() {
    let context = Arc::new(ServerContext {});
    let app = create_router();
    let app = app.with_state(context);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn create_router() -> Router<Arc<ServerContext>> {
    let mut router = Router::new();
    router = router::default::setup_routes(router);
    router = router::get_index::setup_routes(router);
    router = router::get_module_handles::setup_routes(router);
    router = router::post_execute::setup_routes(router);
    router = router::get_scene_valid::setup_routes(router);
    router::swagger_ui::setup_routes(router)
}
