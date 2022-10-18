//Messages that flow from CLIENT to SERVER

use actix::Recipient;
use actix_web::dev::Server;
use rustube::video_info::player_response::streaming_data::QualityLabel;
use serde::{Serialize, Deserialize};
use serde_with::serde_as;
use crate::DownloadSession;
use crate::websocket::server_messages::ServerMessage;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ClientMessageType {
    StartDownload,
}

#[derive(Serialize, Deserialize, Debug, actix::Message)]
#[serde_as]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct StartDownloadMessage {
  pub link: String,
  pub quality_label: QualityLabel, 
  #[serde(skip_deserializing)]
  pub session_id: u64
}

#[derive(actix::Message, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub message_type: ClientMessageType,
    pub message_body: String,
}