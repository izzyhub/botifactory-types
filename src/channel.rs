use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelBody {
    pub channel: ChannelJson,
}

#[derive(Serialize, Deserialize)]
pub struct ChannelJson {
    pub id: i64,
    pub name: String,
    pub project_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Serialize, Deserialize)]
pub struct CreateChannel {
    pub channel_name: String,
}

impl CreateChannel {
    pub fn new(channel_name: &str) -> Self {
        CreateChannel {
            channel_name: channel_name.to_string(),
        }
    }
}
