//Messages that flow from SERVER to CLIENT

use rustube::video_info::player_response::streaming_data::QualityLabel;
use serde::{Serialize, Deserialize};
use serde_with::serde_as;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ServerMessageType {
    SessionCreated,
    DownloadStarted,
    DownloadProgress,
    DownloadFinished,
}


#[derive(Serialize, Deserialize, Debug, actix::Message)]
#[serde_as]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct OnSessionCreated {
    #[serde_as(as = "DisplayFromStr")]
    pub session_id: u64,
}

#[derive(Serialize, Deserialize, Debug, actix::Message)]
#[rtype(result = "()")]
#[serde(rename_all = "camelCase")]
pub struct OnDownloadStarted {
    pub video_link: String,
    pub quality_label: QualityLabel,
    #[serde(skip_serializing)]
    pub session_id: u64,
}

#[derive(Serialize, Deserialize, Debug, actix::Message)]
#[rtype(result = "()")]
#[serde(rename_all = "camelCase")]
pub struct OnDownloadProgress {
    pub progress: f64,
    #[serde(skip_serializing)]
    pub session_id: u64,
    pub video_link: String,
    pub quality_label: QualityLabel,
}

#[derive(Serialize, Deserialize, Debug, actix::Message)]
#[rtype(result = "()")]
#[serde(rename_all = "camelCase")]
pub struct OnDownloadFinished {
    pub video_link: String,
    #[serde(skip_serializing)]
    pub session_id: u64,
    pub quality_label: QualityLabel,
    pub download_link: String,
}

#[derive(actix::Message, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct ServerMessage {
    pub message_type: ServerMessageType,
    pub message_body: String,
}
