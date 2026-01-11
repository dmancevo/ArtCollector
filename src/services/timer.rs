use crate::models::GameState;
use crate::services::game_engine;
use crate::state::AppState;
use std::time::Duration;
use tokio::time;

/// Starts a background task that checks for expired timers every second
pub async fn start_timer_checker(state: AppState) {
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(1));

        loop {
            interval.tick().await;
            check_expired_timers(&state).await;
        }
    });
}

async fn check_expired_timers(state: &AppState) {
    // First, broadcast timer updates for all active games
    {
        let games = state.games.read().await;
        for (game_id, game) in games.iter() {
            if let GameState::Active { timer_ends_at, .. } = &game.state {
                // Skip games waiting for host to start
                if timer_ends_at.is_none() {
                    continue;
                }

                // Broadcast timer update trigger every second
                state
                    .broadcast_sse(game_id, "timer-update", String::from("<!-- trigger -->"))
                    .await;
            }
        }
    }

    // Then check for expired timers
    let games_to_resolve: Vec<String> = {
        let games = state.games.read().await;
        games
            .iter()
            .filter_map(|(id, game)| {
                if let GameState::Active { timer_ends_at, .. } = &game.state {
                    // Only resolve if timer is running and expired
                    if timer_ends_at.is_some() && game.calculate_remaining_seconds() <= 0 {
                        return Some(id.clone());
                    }
                }
                None
            })
            .collect()
    };

    // Resolve rounds for expired games
    for game_id in games_to_resolve {
        resolve_game_round(state, &game_id).await;
    }
}

async fn resolve_game_round(state: &AppState, game_id: &str) {
    // First, resolve the round and check if game finished
    let (is_finished, host_id, winner_ids, final_scores) = {
        let mut games = state.games.write().await;

        if let Some(game) = games.get_mut(game_id) {
            // Resolve the round
            if let Err(e) = game_engine::resolve_round(game) {
                tracing::error!("Failed to resolve round for game {}: {}", game_id, e);
                return;
            }

            // Check if game finished and extract needed data
            let is_finished = matches!(game.state, GameState::Finished { .. });
            if is_finished {
                if let GameState::Finished {
                    winner_ids,
                    final_scores,
                    ..
                } = &game.state
                {
                    (
                        true,
                        game.host_id.clone(),
                        winner_ids.clone(),
                        final_scores.clone(),
                    )
                } else {
                    (false, String::new(), Vec::new(), Vec::new())
                }
            } else {
                (false, String::new(), Vec::new(), Vec::new())
            }
        } else {
            return;
        }
    };

    // If game finished, create a new game for "Play Again"
    if is_finished {
        let mut games = state.games.write().await;

        // Create new empty game with same host (players will join when they click "Play Again")
        let new_game_id = nanoid::nanoid!(21);
        let new_game = crate::models::Game::new(new_game_id.clone(), host_id.clone());

        // Store new game (empty, players join by clicking "Play Again")
        games.insert(new_game_id.clone(), new_game);

        // Update finished game with next_game_id
        if let Some(game) = games.get_mut(game_id) {
            game.state = GameState::Finished {
                winner_ids,
                final_scores,
                next_game_id: Some(new_game_id),
            };
        }
    }

    // Broadcast updates
    let mut games = state.games.write().await;
    if let Some(game) = games.get_mut(game_id) {
        let is_finished = matches!(game.state, GameState::Finished { .. });

        // Broadcast round resolved (new art piece, updated collections)
        let art_html = crate::handlers::partials::render_current_art_partial(game);
        let bid_html = crate::handlers::partials::render_bid_partial(game);

        drop(games);

        state
            .broadcast_sse(game_id, "round-resolved", art_html)
            .await;
        state.broadcast_sse(game_id, "bid-placed", bid_html).await;
        state
            .broadcast_sse(game_id, "timer-update", String::from("<!-- trigger -->"))
            .await;
        // Trigger player info refresh for all players (each will fetch their own data)
        state
            .broadcast_sse(game_id, "player-updated", String::from("<!-- update -->"))
            .await;
        state
            .broadcast_sse(game_id, "bidding-updated", String::from("<!-- trigger -->"))
            .await;

        // If game finished, broadcast redirect
        if is_finished {
            let trigger_html = crate::handlers::partials::render_game_finished_trigger();
            state
                .broadcast_sse(game_id, "game-finished", trigger_html)
                .await;
        }

        tracing::info!("Resolved round for game: {}", game_id);
    }
}
