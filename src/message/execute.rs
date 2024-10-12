use crate::message::execute_log::ExecuteLog;
use crate::message::module_status::ModuleStatus;
use crate::message::scene::Scene;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// 请求执行的结构
#[derive(ToSchema, Serialize, Deserialize)]
pub struct ExecuteReq {
    /// 发起请求时的场景状态
    scene: Scene,
    /// 需要发送拥有“密钥设置”功能的模块的状态，包含字段crypt_key
    #[serde(skip_serializing_if = "Option::is_none")]
    module_status: Option<Vec<ModuleStatus>>,
}

/// 执行返回的结构
#[derive(ToSchema, Serialize, Deserialize)]
pub struct ExecuteRsp {
    /// 执行结束的ExecuteLog，表明路由的顺序
    log: Vec<ExecuteLog>,
    /// 包含拥有“文件查看”及“攻击信息”功能的模块的状态，包含字段file及attack_info
    #[serde(skip_serializing_if = "Option::is_none")]
    module_status: Option<Vec<ModuleStatus>>,
}
