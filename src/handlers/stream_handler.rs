use actix_web::{web, HttpResponse};

pub async fn video_stream() -> HttpResponse {
    HttpResponse::Ok().body("Streaming video") // This will later stream video data
}