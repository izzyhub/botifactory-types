use display_json::DisplayAsJsonPretty;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, DisplayAsJsonPretty)]
pub struct ReleaseResponse {
    pub id: i64,
    pub version: String,
    pub hash: Vec<u8>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Serialize, Deserialize, DisplayAsJsonPretty)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseBody {
    pub release: ReleaseResponse,
}
