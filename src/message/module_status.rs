use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// 模块状态数据：模块的设定，包括文件内容、设置密钥、攻击信息
#[derive(ToSchema, Serialize, Deserialize)]
pub struct ModuleStatus {
    /// 模块的全名
    #[schema(example = "AliceBob.base64.transfer.file_dealer")]
    pub full_name: String,
    /// 加密密钥，请求信息中包含
    #[schema(example = "0123456789abcdef0123456789abcdef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crypt_key: Option<String>,
    /// 文件内容，返回信息中包含
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<File>,
    /// 攻击信息，返回信息中包含
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_info: Option<AttackInfo>,
    /// 是否需要查看，返回信息中包含
    #[schema(example = true)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_to_view: Option<bool>,
}

/// 描述文件的数据结构
#[derive(ToSchema, Serialize, Deserialize)]
pub struct File {
    /// 文件名
    #[schema(example = "test file")]
    name: String,
    /// 文件内容
    #[schema(example = "context of file")]
    text: String,
}

/// 描述攻击信息的数据结构
#[derive(ToSchema, Serialize, Deserialize)]
pub struct AttackInfo {
    /// 攻击结果
    #[schema(example = Success)]
    result: AttackResult,
    /// 攻击时间
    #[schema(example = 66.6)]
    time: f64,
}

/// 描述攻击结果的枚举
#[derive(ToSchema, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttackResult {
    /// 成功
    Success,
    /// 失败
    Failure,
}
