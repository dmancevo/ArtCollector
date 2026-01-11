use axum_test::TestServer;
use collector::routes::create_router;
use collector::state::AppState;

/// Create a test server with a fresh AppState
pub fn create_test_server() -> TestServer {
    let state = AppState::new();
    let app = create_router(state);
    TestServer::new(app).unwrap()
}

/// Extract location header from response (for redirects)
pub fn extract_location(headers: &axum_test::http::HeaderMap) -> Option<String> {
    headers
        .get("location")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
}

/// Extract game_id from a path like "/lobby/abc123" or "/game/abc123/play"
pub fn extract_game_id_from_path(path: &str) -> Option<String> {
    // Remove query params first
    let path_without_query = path.split('?').next().unwrap_or(path);
    let parts: Vec<&str> = path_without_query.split('/').collect();
    if parts.len() >= 3 {
        Some(parts[2].to_string())
    } else {
        None
    }
}

/// Extract player_id from query parameters in a URL
pub fn extract_player_id_from_url(url: &str) -> Option<String> {
    url.split('?').nth(1).and_then(|query| {
        query
            .split('&')
            .find(|param| param.starts_with("player_id="))
            .map(|param| param.strip_prefix("player_id=").unwrap().to_string())
    })
}
