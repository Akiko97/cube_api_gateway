use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// 场景数据：包括节点数据和边数据
#[derive(Serialize, Deserialize, ToSchema)]
pub struct Scene {
    /// 节点数据: 绘制图形需要所有的节点信息，并提前根据配置注册
    pub nodes: Vec<Node>,
    /// 边数据
    pub edges: Vec<Edge>,
    /// 提示
    #[schema(example = "")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tips: Option<String>,
}

/// 边结构
#[derive(Serialize, Deserialize, ToSchema)]
pub struct Edge {
    /// 全名
    #[schema(example = "AliceBob.force_attack.route<main>")]
    pub full_name: String,
    /// 路由配置
    pub routes: Vec<Route>,
}

/// 路由结构
#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Route {
    /// 外部路由（使用全名）
    #[schema(example = json!([
        "AliceBob.force_attack.sender.request",
        "AliceBob.force_attack.sender.file_dealer"
    ]))]
    pub out_route: Vec<String>,
    /// 进入实例的内部路由（使用全名）：外部路由最后一个实例内部的路由
    #[schema(example = json!([
        "AliceBob.force_attack.start",
        "AliceBob.force_attack.sender"
    ]))]
    pub in_route: Vec<String>,
    /// 该组路由的唯一标识，用于映射 UI 里的样式配置文件
    #[schema(example = "AliceBob.force_attack.start$_$AliceBob.force_attack.sender")]
    #[serde(rename = "route_ui_props")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_ui_props: Option<String>,
    /// 属性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub props: Option<RouteProps>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RouteProps {
    /// 入口？
    #[schema(example = "rightTop_in")]
    pub into: String,
    /// 出口？
    #[schema(example = "leftTop_out")]
    pub out: String,
}

/// 节点结构
#[derive(Serialize, Deserialize, ToSchema)]
pub struct Node {
    /// 全名（全局唯一）
    #[schema(example = "AliceBob.force_attack.sender")]
    pub full_name: String,
    /// 名称
    #[schema(example = "sender")]
    pub name: String,
    /// 中文名称
    #[schema(example = "发送者实例")]
    pub name_cn: String,
    /// 类型
    #[schema(example = Instance)]
    #[serde(rename = "type")]
    pub node_type: Type,
    /// 描述
    #[schema(example = "")]
    pub desc: String,
    /// 内部的模块
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modules: Option<Vec<Module>>,
    /// 切面点信息，用于链接分支路由
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tangents: Option<Vec<Module>>,
}

/// 模块结构
/// 留空则作为占位符，为后面预留添加模块的位置
#[derive(Serialize, Deserialize, ToSchema)]
pub struct Module {
    /// 全名（全局唯一）
    #[schema(example = "AliceBob.force_attack.transfer.file_dealer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    /// 名称
    #[schema(example = "file_dealer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 中文名称
    #[schema(example = "文件分发模块")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_cn: Option<String>,
    /// 类型
    #[schema(example = Module)]
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_type: Option<Type>,
    /// 描述
    #[schema(example = "")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
}

/// 节点类型
#[derive(Serialize, Deserialize, ToSchema)]
pub enum Type {
    /// 实例节点
    #[serde(rename = "INSTANCE")]
    Instance,
    /// 普通节点
    #[serde(rename = "NODE")]
    Node,
    /// 模块节点
    #[serde(rename = "MODULE")]
    Module,
}
