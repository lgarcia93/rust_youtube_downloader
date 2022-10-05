mod downloader;
mod websocket;

use std::future::Future;
use std::path::PathBuf;
use std::ptr::null;
use std::str::FromStr;
use std::sync::Arc;
use actix::{Actor, Addr};
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use rustube::{Callback, CallbackArguments, Error, OnCompleteType, OnProgressType, Stream, url, Video, VideoDetails};
use rustube::stream::callback::OnProgressClosure;
use rustube::url::Url;
use rustube::video_info::player_response::streaming_data::{MimeType, ProjectionType, Quality, SignatureCipher};
use actix_cors::Cors;
use actix_files::NamedFile;
use actix_web_actors::ws;
use serde::{Deserialize};
use crate::websocket::session::DownloadSession;
use crate::websocket::socket_server::{OnClientConnected, OnStartListeningDownloadProgress, SocketServer};

#[derive(Deserialize)]
pub struct VideoInfo {
    pub link: String,
    pub session_id: usize
}

pub async fn download_video(req: HttpRequest, srv: web::Data<Addr<SocketServer>>) -> impl Responder {   
    let params = web::Query::<VideoInfo>::from_query(req.query_string()).unwrap();
       
    let path = downloader::download(params.link.as_str()).await;
    
    srv.get_ref().do_send(OnStartListeningDownloadProgress{ session_id: params.session_id });
    
    ""
    
    //NamedFile::open_async(path.expect("")).await.unwrap()
}

pub async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
}

pub async fn video_info(req: HttpRequest) -> impl Responder {
    let params = web::Query::<VideoInfo>::from_query(req.query_string()).unwrap();
    
    if let Ok(videoDataInfo) = downloader::fetch_video_info(params.link.as_str()).await {
        return HttpResponse::Ok().json(videoDataInfo);
    }
    
    HttpResponse::BadRequest().body("")
}

pub async fn web_socket_connection_handler(req: HttpRequest, stream: web::Payload, srv: web::Data<Addr<SocketServer>>) -> Result<HttpResponse, actix_web::Error> {    
    let download_session = DownloadSession {
        server_ref: srv.get_ref().clone(),
    };
       
    ws::start(
        download_session,
        &req,
        stream,
    )
}

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin();

        let socket_server = SocketServer::new().start();

        App::new()
            .app_data(web::Data::new(socket_server.clone()))
            .wrap(cors)         
            .configure(routes)
    })
        .bind(("0.0.0.0", 5000))?
        .run()
        .await

}

fn routes(app: &mut web::ServiceConfig) {
    app
        .service(web::resource("/download_video").route(web::get().to(download_video)))
        .service(web::resource("/").route(web::get().to(index)))
        .service(web::resource("/video_info").route(web::get().to(video_info)))
        .route("/ws", web::get().to(web_socket_connection_handler));
}

fn main() {
   start().expect("");
}