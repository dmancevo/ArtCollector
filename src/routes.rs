use crate::handlers;
use crate::state::AppState;
use axum::{
    Router,
    routing::{get, post},
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Home page
        .route("/", get(handlers::home::index))
        // Create and join game
        .route("/create", post(handlers::home::create_game))
        .route("/join/:game_id", get(handlers::lobby::join_page))
        .route("/join/:game_id", post(handlers::lobby::join_game))
        // Lobby
        .route("/lobby/:game_id", get(handlers::lobby::lobby_view))
        .route(
            "/lobby/:game_id/configure",
            post(handlers::lobby::configure),
        )
        .route("/lobby/:game_id/start", post(handlers::lobby::start_game))
        // Game view
        .route("/game/:game_id/play", get(handlers::game::game_view))
        .route(
            "/game/:game_id/player-info",
            get(handlers::game::player_info),
        )
        .route(
            "/game/:game_id/collection",
            get(handlers::game::collection_display),
        )
        .route(
            "/game/:game_id/bidding-area",
            get(handlers::game::bidding_area),
        )
        .route(
            "/game/:game_id/timer-display",
            get(handlers::game::timer_display),
        )
        // Game actions
        .route("/game/:game_id/bid", post(handlers::game::place_bid))
        .route("/game/:game_id/pass", post(handlers::game::pass_bid))
        .route(
            "/game/:game_id/start-round",
            post(handlers::game::start_round),
        )
        // SSE
        .route("/game/:game_id/events", get(handlers::sse::event_stream))
        // Results
        .route(
            "/game/:game_id/results",
            get(handlers::results::show_results),
        )
        .route(
            "/game/:new_game_id/play-again",
            get(handlers::results::play_again),
        )
        .with_state(state)
}
