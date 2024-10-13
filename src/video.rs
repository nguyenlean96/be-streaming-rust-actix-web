use actix_web::{HttpRequest, HttpResponse, Error, web};
use tokio::{sync::mpsc, task};
use futures_util::StreamExt;
use crate::server::{StreamServer, VideoChunk, StreamId};

#[allow(unused)]
pub async fn video_stream_broadcaster(
    req: HttpRequest,
    stream: web::Payload,
    stream_id: web::Path<StreamId>,
    server: web::Data<StreamServer>,
) -> Result<HttpResponse, Error> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, stream)?;

    // Channel for sending video chunks to the server
    let (tx, mut rx) = mpsc::unbounded_channel::<VideoChunk>();  // `tx` is the sender

    // Pass a reference to `tx` to `start_stream`
    server.start_stream(stream_id.into_inner(), &tx).await;

    // Process WebSocket messages for the video stream
    task::spawn_local(async move {
        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                actix_ws::Message::Binary(bytes) => {
                    // Forward binary data (video chunk) to the server
                    let chunk = bytes.to_vec();
                    if let Err(e) = tx.send(chunk) {  // Still using the original `tx`
                        log::error!("Error sending video chunk: {:?}", e);
                    }
                }
                actix_ws::Message::Close(_) => {
                    log::info!("Stream ended");
                    break;
                }
                _ => {}
            }
        }
    });

    Ok(response)
}

/// Handle video stream for viewers
pub async fn video_stream_viewer(
    req: HttpRequest,
    stream: web::Payload,
    stream_id: web::Path<StreamId>,
    server: web::Data<StreamServer>,
) -> Result<HttpResponse, Error> {
    let (response, mut session, _) = actix_ws::handle(&req, stream)?;

    // Channel for receiving video chunks from the server
    let (tx, mut rx) = mpsc::unbounded_channel::<VideoChunk>();

    // Add viewer to the stream
    match server.add_viewer(&stream_id.into_inner(), tx).await {
        Ok(_) => log::info!("Viewer added to stream"),
        Err(e) => log::error!("Error adding viewer: {:?}", e),
    }

    // Stream video chunks to the viewer
    tokio::spawn(async move {
        while let Some(chunk) = rx.recv().await {
            if let Err(e) = session.binary(chunk).await {
                log::error!("Error sending video to viewer: {:?}", e);
                break;
            }
        }
    });

    Ok(response)
}