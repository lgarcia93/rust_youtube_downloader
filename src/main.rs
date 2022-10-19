mod downloader;
mod websocket;

use std::future::Future;
use std::io::Read;
use std::path::PathBuf;
use std::ptr::null;
use std::str::FromStr;
use std::sync::Arc;
use actix::{Actor, Addr};
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use rustube::{Callback, CallbackArguments, Error, OnCompleteType, OnProgressType, Stream, url, Video, VideoDetails};
use rustube::stream::callback::OnProgressClosure;
use rustube::url::Url;
use rustube::video_info::player_response::streaming_data::{MimeType, ProjectionType, Quality, QualityLabel, SignatureCipher};
use actix_cors::Cors;
use actix_files::NamedFile;
use actix_web_actors::ws;
use serde::{Deserialize};
use crate::websocket::server_messages::{OnDownloadFinished, OnDownloadProgress};
use crate::websocket::session::DownloadSession;
use crate::websocket::socket_server::{OnClientConnected, SocketServer};
use actix_web::body;
use actix_files as fs;
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadVideoQueryParams {
    pub link: String,
  //  pub quality_label: QualityLabel,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoInfoQueryParams {
    pub link: String,
}

pub async fn download_video(req: HttpRequest, mut srv: web::Data<Addr<SocketServer>>) -> impl Responder {
   let params = web::Query::<DownloadVideoQueryParams>::from_query(req.query_string()).unwrap();
    
    if let Ok(file_content) = std::fs::read(params.link.clone()) {
        let image_content =  actix_web::web::Bytes::from(file_content);

        return HttpResponse::Ok().content_type("application/force-download")
            .append_header(
                (
                    "Content-Disposition",
                    format!("attachment; filename={}.mp4;", params.link.clone())
                 )
            )
            .body(image_content);
    }
    
    return HttpResponse::NoContent().body("");
}

pub async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
}

pub async fn video_info(req: HttpRequest) -> impl Responder {
    let params = web::Query::<VideoInfoQueryParams>::from_query(req.query_string()).unwrap();

    if let Ok(videoDataInfo) = downloader::fetch_video_info(params.link.as_str()).await {
        return HttpResponse::Ok().json(videoDataInfo);
    }

    HttpResponse::BadRequest().body("")
}

pub async fn web_socket_connection_handler(req: HttpRequest, stream: web::Payload, srv: web::Data<Addr<SocketServer>>) -> Result<HttpResponse, actix_web::Error> {
    let download_session = DownloadSession {
        server_ref: srv.get_ref().clone(),
        session_id: 0,
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

        let socket_server = SocketServer::new();
        let socket_server_addr = socket_server.start();

        App::new()
            .app_data(web::Data::new(socket_server_addr.clone()))     
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
        //    .service(web::resource("/").route(web::get().to(index)))
        .service(web::resource("/video_info").route(web::get().to(video_info)))      
        .route("/ws", web::get().to(web_socket_connection_handler))
        .service(
            fs::Files::new("/", "./static")
                //.show_files_listing()
                .index_file("index.html")
                .use_last_modified(true),
        );
}

fn main() {
    start().expect("");
}