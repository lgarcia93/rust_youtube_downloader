use actix::{Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, ContextFutureSpawner, fut, Handler, Running, StreamHandler, WrapFuture};
use actix_web_actors::ws;
use actix_web_actors::ws::{Message, ProtocolError};
use crate::{OnClientConnected, SocketServer, websocket};
use crate::websocket::server_messages::ServerMessage;


#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct OnServerGeneratedId(pub String);

pub struct DownloadSession {
    pub server_ref : Addr<SocketServer>,
}

impl Actor for DownloadSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
              
        self.server_ref.do_send(OnClientConnected{ addr: addr.recipient() });
    }
}

//Handle Message from the SocketServer, which can then be sent to the client.
impl Handler<ServerMessage> for DownloadSession {
    type Result = ();

    fn handle(&mut self, msg: ServerMessage, ctx: &mut Self::Context) {
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

                print!("Message received: {}", m);

               // self.addr.do_send(server::OnMessageReceived{content: m.to_string()});
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