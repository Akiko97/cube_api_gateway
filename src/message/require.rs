use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(ToSchema)]
#[derive(Serialize, Deserialize)]
pub struct GetScene {
    pub user_name: String,
    pub scene_id: u32,
}
