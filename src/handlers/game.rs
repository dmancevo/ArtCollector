use crate::{
    models::{ArtPiece, GameState},
    state::AppState,
};
use askama::Template;
use axum::{
    Form,
    extract::{Path, State},
    response::{Html, IntoResponse, Redirect, Response},
};
use serde::Deserialize;

#[derive(Template)]
#[template(path = "game.html")]
pub struct GameTemplate {
    game_id: String,
    player_id: String,
    player_name: String,
    player_chips: i32,
    round: usize,
    current_art: ArtPiece,
    current_highest_bid: u32,
    current_highest_bidder: String,
    has_bid: bool,
    timer_seconds: i64,
    collection_count: usize,
    collection_score: u32,
    is_host: bool,
}

#[derive(Deserialize)]
pub struct GameQuery {
    player_id: String,
}

pub async fn game_view(
    Path(game_id): Path<String>,
    axum::extract::Query(GameQuery { player_id }): axum::extract::Query<GameQuery>,
    State(state): State<AppState>,
) -> Response {
    let games = state.games.read().await;
    let game = match games.get(&game_id) {
        Some(g) => g,
        None => return Redirect::to("/").into_response(),
    };

    // Check if game is finished
    if matches!(game.state, GameState::Finished { .. }) {
        return Redirect::to(&format!(
            "/game/{}/results?player_id={}",
            game_id, player_id
        ))
        .into_response();
    }

    // Check if game is active
    let (round, _timer_ends_at) = match &game.state {
        GameState::Active {
            round,
            timer_ends_at,
        } => (*round, *timer_ends_at),
        _ => {
            return Redirect::to(&format!("/lobby/{}?player_id={}", game_id, player_id))
                .into_response();
        }
    };

    let player = match game.players.get(&player_id) {
        Some(p) => p,
        None => return Redirect::to("/").into_response(),
    };

    let current_art = match &game.current_art {
        Some(art) => art.clone(),
        None => return Redirect::to("/").into_response(),
    };

    let highest_bid = game.get_highest_bid();
    let has_bid = highest_bid.is_some();
    let current_highest_bid = highest_bid.as_ref().map(|b| b.amount).unwrap_or(0);
    let current_highest_bidder = highest_bid
        .and_then(|b| game.players.get(&b.player_id).map(|p| p.name.clone()))
        .unwrap_or_default();

    let timer_seconds = game.calculate_remaining_seconds();

    let is_host = game.is_host(&player_id);

    let template = GameTemplate {
        game_id,
        player_id,
        player_name: player.name.clone(),
        player_chips: player.chips,
        round,
        current_art,
        current_highest_bid,
        current_highest_bidder,
        has_bid,
        timer_seconds,
        collection_count: player.collection.len(),
        collection_score: player.calculate_score(),
        is_host,
    };

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            tracing::error!("Template rendering error: {}", e);
            Html(format!("Template error: {}", e)).into_response()
        }
    }
}

#[derive(Deserialize)]
pub struct BidForm {
    player_id: String,
    amount: u32,
}

pub async fn place_bid(
    Path(game_id): Path<String>,
    State(state): State<AppState>,
    Form(form): Form<BidForm>,
) -> Result<&'static str, Html<String>> {
    let mut games = state.games.write().await;
    let game = match games.get_mut(&game_id) {
        Some(g) => g,
        None => return Err(Html("Game not found".to_string())),
    };

    match crate::services::game_engine::place_bid(game, &form.player_id, form.amount) {
        Ok(_) => {
            // Broadcast bid update and player info via SSE
            let bid_html = crate::handlers::partials::render_bid_partial(game);

            drop(games);

            state.broadcast_sse(&game_id, "bid-placed", bid_html).await;
            // Trigger player info refresh for all players (each will fetch their own data)
            state
                .broadcast_sse(&game_id, "player-updated", String::from("<!-- update -->"))
                .await;
            state
                .broadcast_sse(&game_id, "timer-update", String::from("<!-- trigger -->"))
                .await;
            state
                .broadcast_sse(
                    &game_id,
                    "bidding-updated",
                    String::from("<!-- trigger -->"),
                )
                .await;

            Ok("OK")
        }
        Err(e) => Err(Html(e)),
    }
}

#[derive(Deserialize)]
pub struct PassForm {
    #[allow(dead_code)]
    player_id: String,
}

pub async fn pass_bid(
    Path(_game_id): Path<String>,
    State(_state): State<AppState>,
    Form(_form): Form<PassForm>,
) -> &'static str {
    // Pass just means not bidding - no action needed
    "OK"
}

pub async fn player_info(
    Path(game_id): Path<String>,
    axum::extract::Query(GameQuery { player_id }): axum::extract::Query<GameQuery>,
    State(state): State<AppState>,
) -> Html<String> {
    let games = state.games.read().await;
    let game = match games.get(&game_id) {
        Some(g) => g,
        None => return Html("Game not found".to_string()),
    };

    let player = match game.players.get(&player_id) {
        Some(p) => p,
        None => return Html("Player not found".to_string()),
    };

    let round = match &game.state {
        GameState::Active { round, .. } => *round,
        _ => 0,
    };

    let html = crate::handlers::partials::render_player_info_partial(
        game,
        &player_id,
        round,
        &player.name,
    );

    Html(html)
}

pub async fn collection_display(
    Path(game_id): Path<String>,
    axum::extract::Query(GameQuery { player_id }): axum::extract::Query<GameQuery>,
    State(state): State<AppState>,
) -> Html<String> {
    let games = state.games.read().await;
    let game = match games.get(&game_id) {
        Some(g) => g,
        None => return Html("Game not found".to_string()),
    };

    let player = match game.players.get(&player_id) {
        Some(p) => p,
        None => return Html("Player not found".to_string()),
    };

    let html = crate::handlers::partials::render_collection_display_partial(player);

    Html(html)
}

pub async fn bidding_area(
    Path(game_id): Path<String>,
    axum::extract::Query(GameQuery { player_id }): axum::extract::Query<GameQuery>,
    State(state): State<AppState>,
) -> Html<String> {
    let games = state.games.read().await;
    let game = match games.get(&game_id) {
        Some(g) => g,
        None => return Html("Game not found".to_string()),
    };

    let html = crate::handlers::partials::render_bidding_area_partial(game, &player_id, &game_id);

    Html(html)
}

pub async fn timer_display(
    Path(game_id): Path<String>,
    axum::extract::Query(GameQuery { player_id }): axum::extract::Query<GameQuery>,
    State(state): State<AppState>,
) -> Html<String> {
    let games = state.games.read().await;
    let game = match games.get(&game_id) {
        Some(g) => g,
        None => return Html("Game not found".to_string()),
    };

    let html = crate::handlers::partials::render_timer_partial(game, &player_id, &game_id);

    Html(html)
}

#[derive(Deserialize)]
pub struct StartRoundForm {
    player_id: String,
}

pub async fn start_round(
    Path(game_id): Path<String>,
    State(state): State<AppState>,
    Form(form): Form<StartRoundForm>,
) -> Result<&'static str, Html<String>> {
    let mut games = state.games.write().await;
    let game = match games.get_mut(&game_id) {
        Some(g) => g,
        None => return Err(Html("Game not found".to_string())),
    };

    // Verify the player is the host
    if game.host_id != form.player_id {
        return Err(Html("Only the host can start the round".to_string()));
    }

    match crate::services::game_engine::start_round(game) {
        Ok(_) => {
            drop(games);

            // Broadcast timer update and bidding area update to all players
            state
                .broadcast_sse(&game_id, "timer-update", String::from("<!-- trigger -->"))
                .await;
            state
                .broadcast_sse(
                    &game_id,
                    "bidding-updated",
                    String::from("<!-- trigger -->"),
                )
                .await;

            Ok("OK")
        }
        Err(e) => Err(Html(e)),
    }
}
