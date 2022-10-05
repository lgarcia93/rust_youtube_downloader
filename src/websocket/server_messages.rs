//Messages that flow from SERVER to CLIENT

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessageType {
    OnSessionCreated,
    OnDownloadStarted,
    OnDownloadProgress,
    OnDownloadFinished,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OnSessionCreated {
    pub session_id: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OnDownloadStarted;

#[derive(Serialize, Deserialize, Debug)]
pub struct OnDownloadProgress {
    pub progress: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OnDownloadFinished {
    pub video_link: String
}

#[derive(actix::Message, Serialize, Deserialize, Debug)]
#[rtype(result = "()")]
pub struct ServerMessage {
    pub message_type: ServerMessageType,
    pub message_body: String,
}
