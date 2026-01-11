use super::art::ArtPiece;
use super::bid::Bid;
use super::player::Player;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::task::JoinHandle;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameState {
    Lobby,
    Active {
        round: usize,
        timer_ends_at: Option<DateTime<Utc>>, // None means waiting for host to start
    },
    Finished {
        winner_ids: Vec<String>,          // Support ties
        final_scores: Vec<(String, u32)>, // (player_id, score)
        next_game_id: Option<String>,     // Pre-created game for "Play Again"
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub starting_chips: u32,
    pub bid_timer_seconds: u64,
    pub num_rounds: usize,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            starting_chips: 100,
            bid_timer_seconds: 30,
            num_rounds: 10, // Default to 10 rounds
        }
    }
}

pub struct Game {
    #[allow(dead_code)]
    pub id: String,
    pub host_id: String,
    pub state: GameState,
    pub config: GameConfig,
    pub players: HashMap<String, Player>,
    pub deck: Vec<ArtPiece>,
    pub current_art: Option<ArtPiece>,
    pub current_bids: Vec<Bid>,
    pub discard_pile: Vec<ArtPiece>,
    #[allow(dead_code)]
    pub timer_handle: Option<JoinHandle<()>>,
}

impl Game {
    pub fn new(id: String, host_id: String) -> Self {
        Self {
            id,
            host_id,
            state: GameState::Lobby,
            config: GameConfig::default(),
            players: HashMap::new(),
            deck: Vec::new(),
            current_art: None,
            current_bids: Vec::new(),
            discard_pile: Vec::new(),
            timer_handle: None,
        }
    }

    pub fn add_player(&mut self, player: Player) -> Result<(), String> {
        if !matches!(self.state, GameState::Lobby) {
            return Err("Game already started".to_string());
        }

        if self.players.contains_key(&player.id) {
            return Err("Player already in game".to_string());
        }

        self.players.insert(player.id.clone(), player);
        Ok(())
    }

    pub fn is_host(&self, player_id: &str) -> bool {
        self.host_id == player_id
    }

    pub fn deal_next_art(&mut self) {
        self.current_art = self.deck.pop();
    }

    pub fn get_highest_bid(&self) -> Option<&Bid> {
        self.current_bids.iter().max_by_key(|b| b.amount)
    }

    pub fn calculate_remaining_seconds(&self) -> i64 {
        if let GameState::Active { timer_ends_at, .. } = &self.state {
            match timer_ends_at {
                Some(ends_at) => (ends_at.timestamp() - Utc::now().timestamp()).max(0),
                None => -1, // -1 indicates waiting for host to start round
            }
        } else {
            0
        }
    }
}
