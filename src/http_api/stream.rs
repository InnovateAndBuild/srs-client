use super::common::{Kbps, Publish};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Video {
    pub codec: String,
    pub profile: String,
    pub level: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Audio {
    pub codec: String,
    pub sample_rate: i64,
    pub channel: i64,
    pub profile: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stream {
    pub id: String,
    pub name: String,
    pub vhost: String,
    pub app: String,
    #[serde(rename = "tcUrl")]
    pub tc_url: String,
    pub url: String,
    pub live_ms: i64,
    pub clients: i64,
    pub frames: i64,
    pub send_bytes: i64,
    pub recv_bytes: i64,
    pub kbps: Kbps,
    pub publish: Publish,
    pub video: Option<Video>,
    pub audio: Option<Audio>,
}
