use crate::{models::GameState, state::AppState};
use askama_axum::Template;
use axum::{
    extract::{Path, State},
    response::{Html, Redirect},
};
use serde::Deserialize;

#[derive(Template)]
#[template(path = "results.html")]
pub struct ResultsTemplate {
    #[allow(dead_code)]
    game_id: String,
    player_id: String,
    is_winner: bool,
    final_scores: Vec<PlayerScore>,
    next_game_id: Option<String>,
}

pub struct PlayerScore {
    pub name: String,
    pub score: u32,
    pub is_winner: bool,
    pub collection_count: usize,
}

#[derive(Deserialize)]
pub struct ResultsQuery {
    player_id: String,
}

pub async fn show_results(
    Path(game_id): Path<String>,
    State(state): State<AppState>,
    axum::extract::Query(ResultsQuery { player_id }): axum::extract::Query<ResultsQuery>,
) -> Result<ResultsTemplate, Html<String>> {
    let games = state.games.read().await;
    let game = match games.get(&game_id) {
        Some(g) => g,
        None => return Err(Html("Game not found".to_string())),
    };

    // Check if game is finished
    let (winner_ids, scores, next_game_id) = match &game.state {
        GameState::Finished {
            winner_ids,
            final_scores,
            next_game_id,
        } => (
            winner_ids.clone(),
            final_scores.clone(),
            next_game_id.clone(),
        ),
        _ => {
            return Err(Html("Game not finished yet".to_string()));
        }
    };

    let is_winner = winner_ids.contains(&player_id);

    let final_scores: Vec<PlayerScore> = scores
        .iter()
        .map(|(pid, score)| {
            let player = game.players.get(pid).unwrap();
            PlayerScore {
                name: player.name.clone(),
                score: *score,
                is_winner: winner_ids.contains(pid),
                collection_count: player.collection.len(),
            }
        })
        .collect();

    Ok(ResultsTemplate {
        game_id,
        player_id,
        is_winner,
        final_scores,
        next_game_id,
    })
}

#[derive(Deserialize)]
pub struct PlayAgainQuery {
    player_id: String,
    old_game_id: String,
}

pub async fn play_again(
    Path(new_game_id): Path<String>,
    State(state): State<AppState>,
    axum::extract::Query(query): axum::extract::Query<PlayAgainQuery>,
) -> Result<axum::response::Redirect, Html<String>> {
    let mut games = state.games.write().await;

    // Get player info from old game
    let (player_id, player_name) = {
        let old_game = match games.get(&query.old_game_id) {
            Some(g) => g,
            None => return Err(Html("Original game not found".to_string())),
        };

        let old_player = match old_game.players.get(&query.player_id) {
            Some(p) => p,
            None => return Err(Html("Player not found in original game".to_string())),
        };

        (old_player.id.clone(), old_player.name.clone())
    };

    // Get new game and add player
    let game = match games.get_mut(&new_game_id) {
        Some(g) => g,
        None => return Err(Html("New game not found".to_string())),
    };

    // Get starting chips from new game config
    let starting_chips = game.config.starting_chips;

    // Create fresh player
    let player = crate::models::Player::new(player_id.clone(), player_name, starting_chips);

    // Add player to new game
    match game.add_player(player) {
        Ok(_) => {
            // Broadcast player list update
            let players_html = crate::handlers::partials::render_lobby_players_card(game);
            let host_settings_html = crate::handlers::partials::render_lobby_settings_host(
                game,
                &new_game_id,
                &game.host_id,
            );
            let player_settings_html =
                crate::handlers::partials::render_lobby_settings_player(game);

            drop(games);

            state
                .broadcast_sse(&new_game_id, "lobby-players-updated", players_html)
                .await;
            state
                .broadcast_sse(&new_game_id, "lobby-settings-host", host_settings_html)
                .await;
            state
                .broadcast_sse(&new_game_id, "lobby-settings-player", player_settings_html)
                .await;

            Ok(Redirect::to(&format!(
                "/lobby/{}?player_id={}",
                new_game_id, player_id
            )))
        }
        Err(e) => Err(Html(e)),
    }
}
