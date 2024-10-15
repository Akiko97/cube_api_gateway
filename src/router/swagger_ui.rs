use std::sync::Arc;
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::ServerContext;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::router::get_index::handle_get_index,
        crate::router::get_scene_valid::handle_get_valid,
        crate::router::get_module_handles::handle_get_module_handles,
        crate::router::post_execute::handle_execute,
    ),
    components(
        schemas(crate::message::scene::Scene),
        schemas(crate::message::scene::Edge),
        schemas(crate::message::scene::Route),
        schemas(crate::message::scene::Node),
        schemas(crate::message::scene::Module),
        schemas(crate::message::scene::Type),
        schemas(crate::message::module_handles::ModuleHandles),
        schemas(crate::message::module_handles::Handle),
    )
)]
struct ApiDoc;

pub fn setup_routes(router: Router<Arc<ServerContext>>) -> Router<Arc<ServerContext>> {
    router.merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
}
