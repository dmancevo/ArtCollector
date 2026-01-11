use crate::state::AppState;
use axum::{
    extract::{Path, State},
    response::sse::{Event, KeepAlive, Sse},
};
use futures::stream::Stream;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::BroadcastStream;

#[derive(Serialize, Deserialize)]
pub struct BroadcastMessage {
    pub event_type: String,
    pub html_content: String,
}

pub async fn event_stream(
    Path(game_id): Path<String>,
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    // Get or create broadcast channel for this game
    let tx = state.get_or_create_channel(&game_id).await;
    let rx = tx.subscribe();

    // Convert broadcast receiver to stream
    let stream = BroadcastStream::new(rx).filter_map(|msg| match msg {
        Ok(data) => {
            // Parse broadcast message
            match serde_json::from_str::<BroadcastMessage>(&data) {
                Ok(broadcast) => Some(Ok(Event::default()
                    .event(broadcast.event_type)
                    .data(broadcast.html_content))),
                Err(e) => {
                    tracing::warn!("Failed to parse SSE message: {}", e);
                    None
                }
            }
        }
        Err(e) => {
            tracing::warn!("Broadcast receive error: {}", e);
            None
        }
    });

    Sse::new(stream).keep_alive(KeepAlive::default())
}
