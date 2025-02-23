use alloc::string::String;
use serde::{Deserialize, Serialize};
use semver::Version;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReleaseResponse {
    pub id: i64,
    pub version: Version,
    pub hash: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseBody {
    pub release: ReleaseResponse,
}
