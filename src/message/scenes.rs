use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(ToSchema)]
#[derive(Serialize, Deserialize)]
pub struct SceneRsp {
    pub nodes: Vec<NodeRsp>,
    pub edges: Vec<EdgeRsp>,
}

#[derive(ToSchema)]
#[derive(Serialize, Deserialize)]
pub struct EdgeRsp {
    pub full_name: String,
    pub routes: Vec<RouteRsp>,
}

#[derive(ToSchema)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteRsp {
    pub out_route: Vec<String>,
    pub in_route: Vec<String>,
    #[serde(rename = "route_ui_props")]
    pub route_ui_props: Option<String>,
    pub props: Option<PropsRsp>,
}

#[derive(ToSchema)]
#[derive(Serialize, Deserialize)]
pub struct PropsRsp {
    pub into: String,
    pub out: String,
}

#[derive(ToSchema)]
#[derive(Serialize, Deserialize)]
pub struct NodeRsp {
    pub full_name: String,
    pub name: String,
    pub name_cn: String,
    #[serde(rename = "type")]
    pub node_type: NodeTypeRsp,
    pub desc: String,
    pub modules: Option<Vec<ModuleRsp>>,
    pub tangents: Option<Vec<ModuleRsp>>,
}

#[derive(ToSchema)]
#[derive(Serialize, Deserialize)]
pub enum NodeTypeRsp {
    #[serde(rename = "INSTANCE")]
    Instance,
    #[serde(rename = "NODE")]
    Node,
}

#[derive(ToSchema)]
#[derive(Serialize, Deserialize)]
pub struct ModuleRsp {
    pub full_name: Option<String>,
    pub name: Option<String>,
    pub name_cn: Option<String>,
    #[serde(rename = "type")]
    pub module_type: Option<ModuleTypeRsp>,
    pub desc: Option<String>,
}

#[derive(ToSchema)]
#[derive(Serialize, Deserialize)]
pub enum ModuleTypeRsp {
    #[serde(rename = "MODULE")]
    Module,
}
