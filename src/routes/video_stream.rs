use actix_web::{web, HttpResponse, Responder};

// Handler to start the video stream for a room
async fn start_video_stream(room_id: web::Path<String>) -> impl Responder {
    // Start the video stream using FFmpeg or another video handling tool
    let result = start_stream_for_room(&room_id).await;

    match result {
        Ok(_) => HttpResponse::Ok().body(format!("Started video stream for room: {}", room_id)),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error starting video stream: {}", e))
        }
    }
}

// Handler to stop the video stream for a room
async fn stop_video_stream(room_id: web::Path<String>) -> impl Responder {
    // Stop the video stream for the room
    let result = stop_stream_for_room(&room_id).await;

    match result {
        Ok(_) => HttpResponse::Ok().body(format!("Stopped video stream for room: {}", room_id)),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error stopping video stream: {}", e))
        }
    }
}

// Handler to get the video stream for a room
async fn get_video_stream(room_id: web::Path<String>) -> impl Responder {
    // In a real scenario, serve an HLS (.m3u8) playlist or video stream
    let stream_url = format!("http://your_server/video/{}.m3u8", room_id);
    HttpResponse::Ok().body(format!("Serving video stream: {}", stream_url))
}

// Helper functions for stream management (you can replace these with actual FFmpeg commands or streaming logic)

async fn start_stream_for_room(room_id: &str) -> Result<(), String> {
    // Here, you would start an actual FFmpeg process or any video streaming logic
    println!("Starting stream for room: {}", room_id);
    Ok(())
}

async fn stop_stream_for_room(room_id: &str) -> Result<(), String> {
    // Stop the actual stream (e.g., killing the FFmpeg process)
    println!("Stopping stream for room: {}", room_id);
    Ok(())
}

pub fn video_stream_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/video")
            .route("/start/{room_id}", web::post().to(start_video_stream))
            .route("/stop/{room_id}", web::post().to(stop_video_stream))
            .route("/stream/{room_id}", web::get().to(get_video_stream)),
    );
}
