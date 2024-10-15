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
    #[serde(rename = "AliceBob.init.route<main>")]
    pub route: String,
    /// 实例
    #[serde(rename = "sender")]
    pub instance: String,
    /// 起点
    #[serde(rename = "request")]
    pub from: String,
    /// 终点
    #[serde(rename = "file_dealer")]
    pub to: String,
    /// Log 的类型
    #[schema(example = FileTrans)]
    #[serde(rename = "type")]
    pub log_type: LogType,
    /// Log 的子类型
    #[schema(example = Request)]
    pub subtype: LogSubType,
    /// Log 的 Flow
    #[schema(example = Query)]
    pub flow: LogFlow,
    /// flag
    #[schema(example = "")]
    pub flag: String,
}

/// Log 的类型
#[derive(ToSchema, Serialize, Deserialize)]
pub enum LogType {
    FileTrans,
}

/// Log 的子类型
#[derive(ToSchema, Serialize, Deserialize)]
pub enum LogSubType {
    #[serde(rename = "REQUEST")]
    Request,
    #[serde(rename = "FILE_DATA")]
    FileData,
    #[serde(rename = "NOTICE")]
    Notice,
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
