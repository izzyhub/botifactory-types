use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChannelBody {
    pub channel: ChannelJson,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelJson {
    pub id: i64,
    pub name: String,
    pub project_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateChannel {
    pub channel_name: String,
}

impl CreateChannel {
    pub fn new(channel_name: String) -> Self {
        CreateChannel { channel_name }
    }
}
