use crate::{models::Player, state::AppState};
use askama_axum::Template;
use axum::{
    Form,
    extract::{Path, State},
    response::{Html, Redirect},
};
use serde::Deserialize;

#[derive(Template)]
#[template(path = "join.html")]
pub struct JoinTemplate {
    game_id: String,
    error: Option<String>,
}

#[derive(Deserialize)]
pub struct JoinForm {
    player_name: String,
}

pub async fn join_page(
    Path(game_id): Path<String>,
    State(state): State<AppState>,
) -> Result<JoinTemplate, Redirect> {
    // Check if game exists
    let games = state.games.read().await;
    if !games.contains_key(&game_id) {
        return Err(Redirect::to("/"));
    }
    drop(games);

    Ok(JoinTemplate {
        game_id: game_id.clone(),
        error: None,
    })
}

pub async fn join_game(
    Path(game_id): Path<String>,
    State(state): State<AppState>,
    Form(form): Form<JoinForm>,
) -> Result<Redirect, Html<String>> {
    let player_name = form.player_name.trim().to_string();

    // Validate name
    if player_name.is_empty() || player_name.len() > 20 {
        return Err(Html("Name must be between 1 and 20 characters".to_string()));
    }

    let mut games = state.games.write().await;
    let game = match games.get_mut(&game_id) {
        Some(g) => g,
        None => return Err(Html("Game not found".to_string())),
    };

    // Generate unique player ID
    let player_id = nanoid::nanoid!(12);

    // Get starting chips from config
    let starting_chips = game.config.starting_chips;

    // Create player
    let player = Player::new(player_id.clone(), player_name, starting_chips);

    // Set host_id if this is the first player
    if game.host_id == "pending" {
        game.host_id = player_id.clone();
    }

    // Add player to game
    match game.add_player(player) {
        Ok(_) => {
            // Broadcast player list update (includes count and button state)
            let players_html = crate::handlers::partials::render_lobby_players_card(game);
            let host_settings_html = crate::handlers::partials::render_lobby_settings_host(
                game,
                &game_id,
                &game.host_id,
            );
            let player_settings_html =
                crate::handlers::partials::render_lobby_settings_player(game);

            drop(games);

            state
                .broadcast_sse(&game_id, "lobby-players-updated", players_html)
                .await;
            state
                .broadcast_sse(&game_id, "lobby-settings-host", host_settings_html)
                .await;
            state
                .broadcast_sse(&game_id, "lobby-settings-player", player_settings_html)
                .await;

            // Store player_id in cookie or session (for now, we'll pass it via query param)
            Ok(Redirect::to(&format!(
                "/lobby/{}?player_id={}",
                game_id, player_id
            )))
        }
        Err(e) => Err(Html(e)),
    }
}

#[derive(Template)]
#[template(path = "lobby.html")]
pub struct LobbyTemplate {
    game_id: String,
    player_id: String,
    is_host: bool,
    players: Vec<PlayerInfo>,
    starting_chips: u32,
    bid_timer_seconds: u64,
    num_rounds: usize,
}

#[derive(Clone)]
pub struct PlayerInfo {
    pub name: String,
    pub is_host: bool,
}

#[derive(Deserialize)]
pub struct LobbyQuery {
    player_id: String,
}

pub async fn lobby_view(
    Path(game_id): Path<String>,
    State(state): State<AppState>,
    axum::extract::Query(LobbyQuery { player_id }): axum::extract::Query<LobbyQuery>,
) -> Result<LobbyTemplate, Redirect> {
    let games = state.games.read().await;
    let game = match games.get(&game_id) {
        Some(g) => g,
        None => return Err(Redirect::to("/")),
    };
    let is_host = game.is_host(&player_id);

    let players: Vec<PlayerInfo> = game
        .players
        .values()
        .map(|p| PlayerInfo {
            name: p.name.clone(),
            is_host: game.is_host(&p.id),
        })
        .collect();

    Ok(LobbyTemplate {
        game_id,
        player_id,
        is_host,
        players,
        starting_chips: game.config.starting_chips,
        bid_timer_seconds: game.config.bid_timer_seconds,
        num_rounds: game.config.num_rounds,
    })
}

#[derive(Deserialize)]
pub struct ConfigForm {
    starting_chips: u32,
    bid_timer_seconds: u64,
    num_rounds: usize,
}

pub async fn configure(
    Path(game_id): Path<String>,
    State(state): State<AppState>,
    Form(form): Form<ConfigForm>,
) -> &'static str {
    let mut games = state.games.write().await;
    if let Some(game) = games.get_mut(&game_id) {
        game.config.starting_chips = form.starting_chips.clamp(10, 1000);
        game.config.bid_timer_seconds = form.bid_timer_seconds.clamp(10, 120);
        game.config.num_rounds = form.num_rounds.clamp(1, 90);

        // Update all players' chips
        for player in game.players.values_mut() {
            player.chips = form.starting_chips as i32;
        }

        // Broadcast config and player updates
        let host_settings_html =
            crate::handlers::partials::render_lobby_settings_host(game, &game_id, &game.host_id);
        let player_settings_html = crate::handlers::partials::render_lobby_settings_player(game);

        drop(games);

        state
            .broadcast_sse(&game_id, "lobby-settings-host", host_settings_html)
            .await;
        state
            .broadcast_sse(&game_id, "lobby-settings-player", player_settings_html)
            .await;
    } else {
        drop(games);
    }
    "OK"
}

pub async fn start_game(
    Path(game_id): Path<String>,
    State(state): State<AppState>,
    axum::extract::Query(LobbyQuery { player_id }): axum::extract::Query<LobbyQuery>,
) -> Result<Redirect, Html<String>> {
    let mut games = state.games.write().await;
    let game = match games.get_mut(&game_id) {
        Some(g) => g,
        None => return Err(Html("Game not found".to_string())),
    };

    // Start the game
    match crate::services::game_engine::start_game(game) {
        Ok(_) => {
            // Broadcast game started (triggers redirect for all clients in lobby)
            let html = crate::handlers::partials::render_game_started_trigger();
            drop(games);
            state.broadcast_sse(&game_id, "game-started", html).await;

            Ok(Redirect::to(&format!(
                "/game/{}/play?player_id={}",
                game_id, player_id
            )))
        }
        Err(e) => Err(Html(e)),
    }
}
