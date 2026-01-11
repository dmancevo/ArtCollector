use crate::models::{Bid, Game, GameState};
use crate::services::art_database::get_game_deck_by_count;
use chrono::{Duration, Utc};

pub fn start_game(game: &mut Game) -> Result<(), String> {
    // Validate we're in lobby state
    if !matches!(game.state, GameState::Lobby) {
        return Err("Game already started".to_string());
    }

    // Validate we have at least 2 players
    if game.players.len() < 2 {
        return Err("Need at least 2 players to start".to_string());
    }

    // Generate deck: num_rounds paintings as configured
    let num_rounds = game.config.num_rounds;
    game.deck = get_game_deck_by_count(num_rounds);

    // Deal first art piece
    game.deal_next_art();

    if game.current_art.is_none() {
        return Err("Failed to deal first art piece".to_string());
    }

    // Transition to Active state (waiting for host to start first round)
    game.state = GameState::Active {
        round: 1,
        timer_ends_at: None, // Host will start the timer manually
    };

    Ok(())
}

pub fn place_bid(game: &mut Game, player_id: &str, amount: u32) -> Result<(), String> {
    // Validate game is active
    if !matches!(game.state, GameState::Active { .. }) {
        return Err("Game is not active".to_string());
    }

    // Check if the round has been started by the host
    if let GameState::Active { timer_ends_at, .. } = &game.state
        && timer_ends_at.is_none()
    {
        return Err("Waiting for host to start the round".to_string());
    }

    // Validate player exists
    let player = game.players.get(player_id).ok_or("Player not found")?;

    // Validate player can afford bid
    if !player.can_bid(amount) {
        return Err("Insufficient chips or invalid bid amount".to_string());
    }

    // Validate bid is higher than current highest
    if let Some(highest_bid) = game.get_highest_bid()
        && amount <= highest_bid.amount
    {
        return Err(format!(
            "Bid must be higher than current bid of {}",
            highest_bid.amount
        ));
    }

    // Add bid
    let bid = Bid {
        player_id: player_id.to_string(),
        amount,
        timestamp: Utc::now(),
    };
    game.current_bids.push(bid);

    // Reset timer (round is already started, just reset the timer)
    let timer_duration = game.config.bid_timer_seconds as i64;
    if let GameState::Active { round, .. } = game.state {
        game.state = GameState::Active {
            round,
            timer_ends_at: Some(Utc::now() + Duration::seconds(timer_duration)),
        };
    }

    Ok(())
}

pub fn resolve_round(game: &mut Game) -> Result<(), String> {
    // Validate game is active
    if !matches!(game.state, GameState::Active { .. }) {
        return Err("Game is not active".to_string());
    }

    // Find highest bid
    let winning_bid = game.get_highest_bid().cloned();

    if let Some(winning_bid) = winning_bid {
        let player = game
            .players
            .get_mut(&winning_bid.player_id)
            .ok_or("Winner not found")?;

        // Deduct chips
        player.chips -= winning_bid.amount as i32;

        // Add art to collection
        if let Some(art) = game.current_art.take() {
            player.collection.push(art);
        }
    } else {
        // No bids - discard art
        if let Some(art) = game.current_art.take() {
            game.discard_pile.push(art);
        }
    }

    // Clear bids
    game.current_bids.clear();

    // Check if game is over
    if game.deck.is_empty() {
        finish_game(game, None)?; // next_game_id will be set by timer service
        return Ok(());
    }

    // Deal next art piece and continue
    game.deal_next_art();

    // Move to next round, waiting for host to start
    if let GameState::Active { round, .. } = game.state {
        game.state = GameState::Active {
            round: round + 1,
            timer_ends_at: None, // Host will start the next round manually
        };
    }

    Ok(())
}

pub fn start_round(game: &mut Game) -> Result<(), String> {
    // Validate game is active and waiting for start
    if let GameState::Active {
        round,
        timer_ends_at,
    } = &game.state
    {
        if timer_ends_at.is_some() {
            return Err("Round already in progress".to_string());
        }

        // Start the timer
        let timer_duration = game.config.bid_timer_seconds as i64;
        game.state = GameState::Active {
            round: *round,
            timer_ends_at: Some(Utc::now() + Duration::seconds(timer_duration)),
        };

        Ok(())
    } else {
        Err("Game is not active".to_string())
    }
}

fn finish_game(game: &mut Game, next_game_id: Option<String>) -> Result<(), String> {
    // Calculate all player scores
    let mut scores: Vec<(String, u32)> = game
        .players
        .values()
        .map(|p| (p.id.clone(), p.calculate_score()))
        .collect();

    // Sort by score descending
    scores.sort_by(|a, b| b.1.cmp(&a.1));

    // Find all winners (handle ties)
    let max_score = scores.first().map(|(_, s)| *s).unwrap_or(0);
    let winner_ids: Vec<String> = scores
        .iter()
        .filter(|(_, s)| *s == max_score)
        .map(|(id, _)| id.clone())
        .collect();

    game.state = GameState::Finished {
        winner_ids,
        final_scores: scores,
        next_game_id,
    };

    Ok(())
}
