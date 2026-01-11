use crate::models::Game;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};

pub type GameId = String;

#[derive(Clone)]
pub struct AppState {
    pub games: Arc<RwLock<HashMap<GameId, Game>>>,
    pub sse_channels: Arc<RwLock<HashMap<GameId, broadcast::Sender<String>>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            games: Arc::new(RwLock::new(HashMap::new())),
            sse_channels: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Get or create an SSE broadcast channel for a game
    pub async fn get_or_create_channel(&self, game_id: &str) -> broadcast::Sender<String> {
        let mut channels = self.sse_channels.write().await;
        channels
            .entry(game_id.to_string())
            .or_insert_with(|| broadcast::channel(100).0)
            .clone()
    }

    /// Broadcast a message to all connected clients for a game
    pub async fn broadcast(&self, game_id: &str, message: String) {
        if let Some(tx) = self.sse_channels.read().await.get(game_id) {
            let _ = tx.send(message);
        }
    }

    /// Broadcast an SSE event with HTML content to all connected clients
    pub async fn broadcast_sse(&self, game_id: &str, event_type: &str, html_content: String) {
        use crate::handlers::sse::BroadcastMessage;

        let message = BroadcastMessage {
            event_type: event_type.to_string(),
            html_content,
        };

        if let Ok(json) = serde_json::to_string(&message) {
            self.broadcast(game_id, json).await;
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
