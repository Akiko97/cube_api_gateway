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

    modules: Option<Vec<Node>>,
}
