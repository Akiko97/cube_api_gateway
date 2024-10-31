use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

/// 场景执行返回的 Log 数据
#[derive(ToSchema, Serialize, Deserialize)]
pub struct ExecuteLog {
    /// 返回的 Log 数据
    pub log: Vec<Log>,
}

/// Log 数据的条目
#[derive(ToSchema, Serialize, Deserialize)]
pub struct Log {
    /// 路径名
    #[schema(example = "AliceBob.init.route<main>")]
    pub route: String,
    /// 实例
    #[schema(example = "sender")]
    pub instance: String,
    /// 起点
    #[schema(example = "request")]
    pub from: String,
    /// 终点
    #[schema(example = "file_dealer")]
    pub to: String,
    /// Log 的 Flow
    #[schema(example = Query)]
    pub flow: LogFlow,
    /// flag
    #[schema(example = "")]
    pub flag: String,
}

/// Log 的 Flow
#[derive(ToSchema, Serialize, Deserialize)]
pub enum LogFlow {
    #[serde(rename = "RESPONSE")]
    Response,
    #[serde(rename = "QUERY")]
    Query,
    #[serde(rename = "DELIVER")]
    Deliver,
    #[serde(rename = "FINISH")]
    Finish,
}
