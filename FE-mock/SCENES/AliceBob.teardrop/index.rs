// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::index;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: index = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Index {
    nodes: Vec<Node>,

    edges: Vec<Edge>,
}

#[derive(Serialize, Deserialize)]
pub struct Edge {
    full_name: String,

    routes: Vec<Route>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Route {
    out_route: Vec<String>,

    in_route: Vec<String>,

    #[serde(rename = "route_ui_props")]
    route_ui_props: String,
}

#[derive(Serialize, Deserialize)]
pub struct Node {
    full_name: String,

    name: String,

    name_cn: String,

    #[serde(rename = "type")]
    node_type: String,

    desc: String,

    modules: Option<Vec<Module>>,

    tangents: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Serialize, Deserialize)]
pub struct Module {
    full_name: Option<String>,

    name: Option<String>,

    name_cn: Option<String>,

    #[serde(rename = "type")]
    module_type: Option<String>,

    desc: Option<String>,
}
