use std::borrow::Borrow;
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;
use actix::{Actor, Context, Handler, Message, Recipient};
use actix_web_actors::ws;
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::DownloadSession;
use crate::websocket::server_messages::{ServerMessage, ServerMessageType, OnSessionCreated};
use crate::websocket::server_messages::ServerMessageType::OnDownloadStarted;

#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct OnClientConnected {
    pub addr: Recipient<ServerMessage>,
}

#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct OnStartListeningDownloadProgress {
    pub session_id: usize,
    pub addr: Recipient<ServerMessage>,
}


pub struct SocketServer {
    rng: ThreadRng,
    pub connected_sessions: HashMap<usize, Recipient<ServerMessage>>,
}

impl SocketServer {
    pub fn new() -> SocketServer {
        SocketServer {
            rng: rand::thread_rng(),
            connected_sessions: HashMap::new(),
        }
    }

    pub fn send_message(&mut self) {
        for (key, recipient) in self.connected_sessions.iter() {
            //   recipient.do_send(Message(" From server".to_string()));
        }
    }
}

impl Actor for SocketServer {
    type Context = Context<Self>;
}


impl Handler<OnClientConnected> for SocketServer {
    type Result = ();

    fn handle(&mut self, msg: OnClientConnected, ctx: &mut Self::Context) -> Self::Result {
        let session_id: usize = self.rng.gen();

        let session_created_message = OnSessionCreated { session_id };
        
        msg.addr.do_send(
            ServerMessage{
            message_type: ServerMessageType::OnSessionCreated, 
            message_body: serde_json::to_string(&session_created_message).unwrap() 
        });

        self.connected_sessions.insert(session_id, msg.addr);
    }
}

impl Handler<OnStartListeningDownloadProgress> for SocketServer {
    type Result = ();

    fn handle(&mut self, msg: OnStartListeningDownloadProgress, ctx: &mut Self::Context) -> Self::Result {
        
        
        for (i,m ) in self.connected_sessions.iter() {
            
        }
        
        let on_download_started = OnDownloadStarted{};
        
        msg.addr.do_send( ServerMessage{
            message_type: ServerMessageType::OnDownloadStarted,
            message_body: serde_json::to_string(&on_download_started).unwrap()
        });
    }
}