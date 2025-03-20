use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TagDocument {
    pub photo_id: String,
    pub user_id: String,
}
