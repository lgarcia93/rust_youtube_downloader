use std::error::Error;
use actix::{Actor, ActorContext, ActorFutureExt, Addr, ArbiterHandle, AsyncContext, Context, ContextFutureSpawner, fut, Handler, Running, StreamHandler, WrapFuture};
use actix_web_actors::ws;
use actix_web_actors::ws::{Message, ProtocolError};
use crate::{OnClientConnected, SocketServer, websocket};
use crate::websocket::client_messages::{ClientMessage, ClientMessageType, StartDownloadMessage};
use crate::websocket::server_messages::{OnSessionCreated, ServerMessage, ServerMessageType};


#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct OnServerGeneratedId(pub String);

pub struct DownloadSession {
    pub server_ref: Addr<SocketServer>,
    pub session_id: u64
}

impl Actor for DownloadSession {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        
        self.server_ref.do_send(OnClientConnected { recipient: addr.clone().recipient()});
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {}
}

//Handle Message from the SocketServer, which can then be sent to the client.
impl Handler<ServerMessage> for DownloadSession {
    type Result = ();

    fn handle(&mut self, msg: ServerMessage, ctx: &mut Self::Context) {
        
        if matches!(msg.message_type, ServerMessageType::SessionCreated) {
            let session_created: OnSessionCreated = serde_json::from_str(&msg.message_body).expect("");
            
            self.session_id = session_created.session_id;
        }
        
        ctx.text(serde_json::to_string(&msg).expect(""));
    }
}

//handles message FROM client TO server
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for DownloadSession {
    fn handle(&mut self, item: Result<Message, ProtocolError>, ctx: &mut Self::Context) {
        let msg = match item {
            Ok(msg) => msg,
            Err(_) => {
                ctx.stop();
                return;
            }
        };

        match msg {
            ws::Message::Text(text) => {
                let m = text.trim();

                let message_res: Result<ClientMessage, serde_json::Error> = serde_json::from_str(m);

                match message_res {
                    Ok(message) => {
                        
                        match message.message_type {
                            ClientMessageType::StartDownload => {
                                let start_download_res: Result<StartDownloadMessage, serde_json::Error> = serde_json::from_str(message.message_body.as_str());
                                
                                match start_download_res{
                                    Ok(mut start_download_msg) => {
                                        start_download_msg.session_id = self.session_id;
                                        
                                        self.server_ref.do_send(start_download_msg);
                                    }
                                    _ => {}
                                }
                            }
                        }
                            
                        
                    }
                    _ => {}
                }

                print!("Message received: {}", m);
            }
            ws::Message::Binary(_) => println!("Server doesn't accept binary"),
            Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Ping(_) => {}
            ws::Message::Pong(_) => {}
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Nop => ()
        }
    }
}