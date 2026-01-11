use super::art::{ArtPiece, Artist, Movement};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub chips: i32, // Allow negative for display, but prevent negative bids
    pub collection: Vec<ArtPiece>,
}

impl Player {
    pub fn new(id: String, name: String, starting_chips: u32) -> Self {
        Self {
            id,
            name,
            chips: starting_chips as i32,
            collection: Vec::new(),
        }
    }

    pub fn calculate_score(&self) -> u32 {
        let artist_score = self.calculate_artist_bonus();
        let movement_score = self.calculate_movement_bonus();
        // Use max, not sum - pieces count for both bonuses but we take the higher one
        artist_score.max(movement_score)
    }

    fn calculate_artist_bonus(&self) -> u32 {
        // Group by artist: sum(stars) × count
        let mut artist_groups: HashMap<Artist, Vec<u8>> = HashMap::new();
        for art in &self.collection {
            artist_groups.entry(art.artist).or_default().push(art.stars);
        }

        artist_groups
            .values()
            .map(|pieces| {
                let sum: u32 = pieces.iter().map(|&s| s as u32).sum();
                let count = pieces.len() as u32;
                sum * count
            })
            .sum()
    }

    fn calculate_movement_bonus(&self) -> u32 {
        // Group by movement: sum(stars) × count
        let mut movement_groups: HashMap<Movement, Vec<u8>> = HashMap::new();
        for art in &self.collection {
            movement_groups
                .entry(art.movement)
                .or_default()
                .push(art.stars);
        }

        movement_groups
            .values()
            .map(|pieces| {
                let sum: u32 = pieces.iter().map(|&s| s as u32).sum();
                let count = pieces.len() as u32;
                sum * count
            })
            .sum()
    }

    pub fn can_bid(&self, amount: u32) -> bool {
        self.chips >= amount as i32 && amount > 0
    }
}
