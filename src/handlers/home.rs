use crate::{models::Game, state::AppState};
use askama_axum::Template;
use axum::{extract::State, response::Redirect};

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate;

pub async fn index() -> HomeTemplate {
    HomeTemplate
}

pub async fn create_game(State(state): State<AppState>) -> Redirect {
    // Generate unique game ID
    let game_id = nanoid::nanoid!(8);

    // For now, use a placeholder host ID (will be replaced when first player joins)
    let game = Game::new(game_id.clone(), "pending".to_string());

    // Store game in state
    state.games.write().await.insert(game_id.clone(), game);

    // Redirect to join page for the creator to enter their name
    Redirect::to(&format!("/join/{}", game_id))
}
