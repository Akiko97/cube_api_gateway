use crate::ServerContext;
use axum::Router;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

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
        schemas(crate::message::scene::RouteProps),
        schemas(crate::message::scene::Node),
        schemas(crate::message::scene::Module),
        schemas(crate::message::scene::Type),
        schemas(crate::message::module_handles::ModuleHandles),
        schemas(crate::message::module_handles::Handle),
        schemas(crate::message::execute_log::ExecuteLog),
        schemas(crate::message::execute_log::Log),
        schemas(crate::message::execute_log::LogFlow),
        schemas(crate::message::module_status::ModuleStatus),
        schemas(crate::message::module_status::File),
        schemas(crate::message::module_status::AttackInfo),
        schemas(crate::message::module_status::AttackResult),
        schemas(crate::message::module_status::AttackData),
        schemas(crate::message::execute::ExecuteReq),
        schemas(crate::message::execute::ExecuteRsp),
    )
)]
struct ApiDoc;

pub fn setup_routes(router: Router<Arc<ServerContext>>) -> Router<Arc<ServerContext>> {
    router.merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
}
