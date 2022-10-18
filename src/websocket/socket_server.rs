use std::borrow::Borrow;
use std::collections::HashMap;
use std::ops::Deref;
use std::thread::sleep;
use std::time::Duration;
use actix::{Actor, Addr, AsyncContext, Context, Handler, Message, Recipient};
use actix_web::dev::Server;
use actix_web_actors::ws;
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::{downloader, DownloadSession, OnDownloadFinished};
use crate::websocket::client_messages::StartDownloadMessage;
use crate::websocket::server_messages::{ServerMessage, ServerMessageType, OnSessionCreated, OnDownloadProgress, OnDownloadStarted};
use crate::websocket::server_messages::ServerMessageType::{DownloadFinished, DownloadProgress, DownloadStarted};

#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct OnClientConnected {
    pub recipient: Recipient<ServerMessage>,
}

pub struct SocketServer {
    rng: ThreadRng,
    pub connected_sessions: HashMap<u64, Recipient<ServerMessage>>,
}

impl SocketServer {
    pub fn new() -> SocketServer {
        SocketServer {
            rng: rand::thread_rng(),
            connected_sessions: HashMap::new(),
        }
    }
    
    pub fn notify_download_started(&mut self, message: OnDownloadStarted) {
        if let Some(session) = self.connected_sessions.get(&message.session_id) {
            session.do_send(
                ServerMessage{ 
                    message_type: DownloadStarted,
                    message_body: serde_json::to_string(&message).unwrap()
                }
            );
        }
    }
    
    pub fn notify_download_finished(&mut self, message: OnDownloadFinished) {
        if let Some(session) = self.connected_sessions.get(&message.session_id) {
            session.do_send(
                ServerMessage{
                    message_type: DownloadFinished,
                    message_body: serde_json::to_string(&message).unwrap()
                }
            );
        }
    }
}

impl Actor for SocketServer {
    type Context = Context<Self>;
}

impl Handler<OnClientConnected> for SocketServer {
    type Result = ();

    fn handle(&mut self, msg: OnClientConnected, ctx: &mut Self::Context) -> Self::Result {
        let session_id: u64 = self.rng.gen();

        let session_created_message = OnSessionCreated { session_id };
        
        msg.recipient.do_send(
            ServerMessage{
            message_type: ServerMessageType::SessionCreated, 
            message_body: serde_json::to_string(&session_created_message).unwrap() 
        });

        self.connected_sessions.insert(session_id, msg.recipient);
    }
}

impl Handler<OnDownloadProgress> for SocketServer {
    type Result = ();

    fn handle(&mut self, msg: OnDownloadProgress, ctx: &mut Self::Context) -> Self::Result {
                if let Some(item) =  self.connected_sessions.get(&msg.session_id){
                    
            item.do_send( ServerMessage{
                message_type: DownloadProgress,
                message_body: serde_json::to_string(&msg).unwrap()
            });
        }
    }
}

impl Handler<OnDownloadFinished> for SocketServer {
    type Result = ();

    fn handle(&mut self, msg: OnDownloadFinished, ctx: &mut Self::Context) -> Self::Result {
        if let Some(item) =  self.connected_sessions.get(&msg.session_id){

            item.do_send( ServerMessage{
                message_type: DownloadFinished,
                message_body: serde_json::to_string(&msg).unwrap()
            });
        }
    }
}

impl Handler<StartDownloadMessage> for SocketServer {
    type Result = ();

    fn handle(&mut self, msg: StartDownloadMessage, ctx: &mut Self::Context) -> Self::Result {
         let rec =  ctx.address().recipient().clone();
        
        let download_started = OnDownloadStarted {
            video_link: msg.link.clone(), 
            quality_label: msg.quality_label,
            session_id: msg.session_id,
        };
        
        self.notify_download_started(download_started);
        
        let link = msg.link.clone();

        if let Some(session) = self.connected_sessions.get(&msg.session_id) {

            let cloned_session = session.clone();
            let _ = actix_web::rt::spawn(async move {
                let path = downloader::download(
                    msg.link.as_str(),
                    msg.quality_label,
                    move |progress| {
                        let download_progress = OnDownloadProgress {
                            progress,
                            session_id: msg.session_id,
                            video_link: link.clone(),
                            quality_label: msg.quality_label,
                        };

                        rec.do_send(
                            download_progress
                        );
                    }).await;

                let message = OnDownloadFinished {
                    video_link: msg.link.to_string(),
                    session_id: msg.session_id,
                    quality_label: msg.quality_label,
                    download_link: path.unwrap().into_os_string().into_string().unwrap()
                };

                cloned_session.do_send(
                    ServerMessage {
                        message_type: DownloadFinished,
                        message_body: serde_json::to_string(&message).unwrap()
                    }
                );
            });
        }
    }
}

