use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(ToSchema)]
#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

#[derive(ToSchema)]
#[derive(Serialize, Deserialize)]
pub struct Edge {
    pub full_name: String,
    pub routes: Vec<Route>,
}

#[derive(ToSchema)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Route {
    pub out_route: Vec<String>,
    pub in_route: Vec<String>,
    #[serde(rename = "route_ui_props")]
    pub route_ui_props: Option<String>,
    pub props: Option<Props>,
}

#[derive(ToSchema)]
#[derive(Serialize, Deserialize)]
pub struct Props {
    pub into: String,
    pub out: String,
}

#[derive(ToSchema)]
#[derive(Serialize, Deserialize)]
pub struct Node {
    pub full_name: String,
    pub name: String,
    pub name_cn: String,
    #[serde(rename = "type")]
    pub node_type: NodeType,
    pub desc: String,
    pub modules: Option<Vec<Module>>,
    pub tangents: Option<Vec<Module>>,
}

#[derive(ToSchema)]
#[derive(Serialize, Deserialize)]
pub enum NodeType {
    #[serde(rename = "INSTANCE")]
    Instance,
    #[serde(rename = "NODE")]
    Node,
}

#[derive(ToSchema)]
#[derive(Serialize, Deserialize)]
pub struct Module {
    pub full_name: Option<String>,
    pub name: Option<String>,
    pub name_cn: Option<String>,
    #[serde(rename = "type")]
    pub module_type: Option<ModuleType>,
    pub desc: Option<String>,
}

#[derive(ToSchema)]
#[derive(Serialize, Deserialize)]
pub enum ModuleType {
    #[serde(rename = "MODULE")]
    Module,
}
