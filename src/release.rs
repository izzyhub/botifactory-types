use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ReleaseResponse {
    pub id: i64,
    pub version: String,
    pub hash: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseBody {
    pub release: ReleaseResponse,
}
