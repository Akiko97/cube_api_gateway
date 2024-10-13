use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

/// 获取场景信息的请求结构体
#[derive(Serialize, Deserialize, ToSchema)]
pub struct GetSceneReq {
    /// 用户名，用于标识请求的用户
    #[schema(example = "zhao")]
    pub user_name: String,
    /// 场景ID，用于指定要获取的场景
    #[schema(example = 0)]
    pub scene_id: u32,
    /// 请求的场景消息类型
    #[schema(example = Index)]
    #[serde(rename = "type")]
    pub req_type: SceneReqType,
}

/// 请求的场景消息类型
#[derive(Serialize, Deserialize, ToSchema)]
pub enum SceneReqType {
    /// 请求场景的 Index
    #[serde(rename = "index")]
    Index,
}
