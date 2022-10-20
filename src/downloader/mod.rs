use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::path::PathBuf;
use actix::dev::Stream;
use rustube::{Callback, CallbackArguments, OnProgressType, Video};
use rustube::video_info::player_response::streaming_data::{Quality, QualityLabel};
use rustube::video_info::player_response::video_details::Thumbnail;
use crate::{SocketServer, Url};
use crate::url::ParseError;
use serde::{Serialize, Deserialize};

pub struct DownloadError;

#[derive(Serialize, Deserialize)]
pub struct VideoDataInfo {
    pub title: String,
    pub format: String,
    pub quality_label: QualityLabel,
    pub quality: Quality,
    pub thumbnails: Vec<Thumbnail>,
    pub link: String
}

impl Debug for DownloadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl Display for DownloadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl Error for DownloadError {}

pub async fn download(raw_url: &str, quality_label: QualityLabel, progress_callback: impl Fn(f64) -> () + 'static) -> Result<PathBuf, Box<dyn Error>> {
    if let Ok(url) = Url::parse(raw_url) {
        if let Ok(video) = Video::from_url(&url).await {
            if let Some(stream_best_quality) = video.streams()
                .iter()
                .filter(|stream| stream.includes_video_track && matches![stream.quality_label.unwrap(), quality_label])
                .next()
                {
                //.min_by_key(|stream| ) {
                let mut callback = Callback::new();

                callback.on_progress = OnProgressType::Closure(Box::from(move |p: CallbackArguments| {
                    
                    if let Some(content_length) = p.content_length {
                        if content_length > 0 {
                            progress_callback(p.current_chunk as f64 / content_length as f64);
                        }
                    }
                }));
                if let Ok(path) = stream_best_quality.download_with_callback(callback).await {
                    return Ok(path);
                }
            }
        }
    }

    Err(Box::new(DownloadError {}))
}

pub async fn fetch_video_info(raw_url: &str) -> Result<Vec<VideoDataInfo>, Box<dyn Error>> {
    if let Ok(url) = Url::parse(raw_url) {        
        if let Ok(video) = Video::from_url(&url).await {
            let video_data_infos: Vec<VideoDataInfo> = video
                .streams()
                .iter()
                .filter(|stream| { stream.includes_video_track})
                .map(|stream| {
                    let quality = stream.quality_label;
                    let mut final_quality = QualityLabel::P144;
                    if let Some(ql) = quality {
                        final_quality = ql;
                    }
                    
                                     
                    return VideoDataInfo {
                        title: stream.video_details.title.clone(),
                        quality_label: final_quality,
                        format: "".to_string(),
                        quality: stream.quality,
                        thumbnails: video.video_details().clone().thumbnails.clone(),
                        link: raw_url.to_string()
                    };
                }).collect();

            return Ok(video_data_infos);
        }
    }

    Ok(vec![])
}