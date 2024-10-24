use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

/// 模块操作数据：返回特定场景内模块可以进行的操作
#[derive(ToSchema, Serialize, Deserialize)]
pub struct ModuleHandles {
    /// 模块的全名
    #[schema(example = "AliceBob.base64.transfer.file_dealer")]
    pub full_name: String,
    /// 模块可以进行的操作
    #[schema(example = json!([
        "show_file",
        "link_module",
        "replace_module",
        "delete_module"
    ]))]
    pub handles: Vec<Handle>,
}

/// 模块可以执行的操作
#[derive(ToSchema, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Handle {
    /// 显示文件内容（文件名和文件内容）
    ShowFile,
    /// 连接到其他模块
    LinkModule,
    /// 替换模块
    ReplaceModule,
    /// 删除模块
    DeleteModule,
    /// 设置密钥（密钥字符串）
    SetKeys,
    /// 攻击信息，获取穷举攻击信息（破译时长等）
    AttackInfo,
}
