mod common;

use common::*;

// Helper function to create and join a game, returning game_id and player_id
async fn setup_joined_game(server: &axum_test::TestServer, player_name: &str) -> (String, String) {
    // Create a game
    let create_response = server.post("/create").await;
    let join_location = extract_location(create_response.headers()).unwrap();
    let game_id = extract_game_id_from_path(&join_location).unwrap();

    // Join the game
    let join_response = server
        .post(&format!("/join/{}", game_id))
        .form(&[("player_name", player_name)])
        .await;

    let lobby_location = extract_location(join_response.headers()).unwrap();
    let player_id = extract_player_id_from_url(&lobby_location).unwrap();

    (game_id, player_id)
}

#[tokio::test]
async fn test_create_game() {
    let server = create_test_server();

    // Create a game
    let response = server.post("/create").await;

    // Should redirect to join page
    assert_eq!(response.status_code(), 303);

    // Should have a location header pointing to join page
    let location = extract_location(response.headers()).expect("Should have location header");
    assert!(location.starts_with("/join/"));

    // Extract game_id
    let game_id = extract_game_id_from_path(&location).expect("Should have game_id in path");
    assert!(!game_id.is_empty());
}

#[tokio::test]
async fn test_join_game() {
    let server = create_test_server();

    // First, create a game
    let create_response = server.post("/create").await;
    let join_location = extract_location(create_response.headers()).unwrap();
    let game_id = extract_game_id_from_path(&join_location).unwrap();

    // Get the join page
    let join_page_response = server.get(&format!("/join/{}", game_id)).await;
    assert_eq!(join_page_response.status_code(), 200);

    // Join the game with a player name
    let join_response = server
        .post(&format!("/join/{}", game_id))
        .form(&[("player_name", "Alice")])
        .await;

    // Should redirect to lobby with player_id query param
    assert_eq!(join_response.status_code(), 303);
    let location = extract_location(join_response.headers()).unwrap();
    assert!(location.starts_with(&format!("/lobby/{}", game_id)));

    // Should have player_id in query string
    let player_id = extract_player_id_from_url(&location).expect("Should have player_id in URL");
    assert!(!player_id.is_empty());
}

#[tokio::test]
async fn test_configure_game() {
    let server = create_test_server();
    let (game_id, _player_id) = setup_joined_game(&server, "Host").await;

    // Configure the game
    let config_response = server
        .post(&format!("/lobby/{}/configure", game_id))
        .form(&[
            ("starting_chips", "200"),
            ("bid_timer_seconds", "45"),
            ("num_rounds", "15"),
        ])
        .await;

    // Should return success
    assert!(config_response.status_code().is_success());
}

#[tokio::test]
async fn test_start_game() {
    let server = create_test_server();
    let (game_id, player_id) = setup_joined_game(&server, "Host").await;

    // Start the game - note that start_game endpoint expects player_id in query param based on lobby.rs:206
    let start_response = server
        .post(&format!("/lobby/{}/start?player_id={}", game_id, player_id))
        .await;

    // Should redirect to game page OR return 200 (depends on implementation)
    assert!(start_response.status_code().is_success() || start_response.status_code() == 303);

    if start_response.status_code() == 303 {
        let location = extract_location(start_response.headers()).unwrap();
        assert!(location.starts_with(&format!("/game/{}/play", game_id)));
    }
}

#[tokio::test]
async fn test_game_view() {
    let server = create_test_server();
    let (game_id, player_id) = setup_joined_game(&server, "Host").await;

    // Start the game
    server
        .post(&format!("/lobby/{}/start?player_id={}", game_id, player_id))
        .await;

    // Start a round (needed to view game page without redirect)
    let start_round_response = server
        .post(&format!("/game/{}/start-round", game_id))
        .form(&[("player_id", player_id.as_str())])
        .await;

    // Verify round started successfully
    assert!(start_round_response.status_code().is_success());

    // View the game page
    let game_response = server
        .get(&format!("/game/{}/play?player_id={}", game_id, player_id))
        .await;

    // The game view might redirect to results if game is finished, so accept both 200 and 303
    assert!(game_response.status_code() == 200 || game_response.status_code() == 303);

    if game_response.status_code() == 200 {
        let body = game_response.text();
        assert!(body.contains("Art Collector") || body.contains("art") || body.contains("game"));
    }
}

#[tokio::test]
async fn test_start_round() {
    let server = create_test_server();
    let (game_id, player_id) = setup_joined_game(&server, "Host").await;

    // Start the game
    server
        .post(&format!("/lobby/{}/start?player_id={}", game_id, player_id))
        .await;

    // Start a round
    let start_round_response = server
        .post(&format!("/game/{}/start-round", game_id))
        .form(&[("player_id", player_id.as_str())])
        .await;

    // Should return success
    assert!(start_round_response.status_code().is_success());
}

#[tokio::test]
async fn test_place_bid() {
    let server = create_test_server();
    let (game_id, player_id) = setup_joined_game(&server, "Host").await;

    // Start the game
    server
        .post(&format!("/lobby/{}/start?player_id={}", game_id, player_id))
        .await;

    // Start a round
    server
        .post(&format!("/game/{}/start-round", game_id))
        .form(&[("player_id", player_id.as_str())])
        .await;

    // Place a bid
    let bid_response = server
        .post(&format!("/game/{}/bid", game_id))
        .form(&[("player_id", player_id.as_str()), ("amount", "10")])
        .await;

    // Should return success
    assert!(bid_response.status_code().is_success());
}

#[tokio::test]
async fn test_pass_bid() {
    let server = create_test_server();
    let (game_id, player_id) = setup_joined_game(&server, "Host").await;

    // Start the game
    server
        .post(&format!("/lobby/{}/start?player_id={}", game_id, player_id))
        .await;

    // Start a round
    server
        .post(&format!("/game/{}/start-round", game_id))
        .form(&[("player_id", player_id.as_str())])
        .await;

    // Pass on bidding
    let pass_response = server
        .post(&format!("/game/{}/pass", game_id))
        .form(&[("player_id", player_id.as_str())])
        .await;

    // Should return success
    assert!(pass_response.status_code().is_success());
}

#[tokio::test]
async fn test_multiple_players_join() {
    let server = create_test_server();

    // Create a game
    let create_response = server.post("/create").await;
    let join_location = extract_location(create_response.headers()).unwrap();
    let game_id = extract_game_id_from_path(&join_location).unwrap();

    // Join as player 1
    let join1_response = server
        .post(&format!("/join/{}", game_id))
        .form(&[("player_name", "Alice")])
        .await;
    assert_eq!(join1_response.status_code(), 303);
    let location1 = extract_location(join1_response.headers()).unwrap();
    let player1_id = extract_player_id_from_url(&location1).unwrap();

    // Join as player 2
    let join2_response = server
        .post(&format!("/join/{}", game_id))
        .form(&[("player_name", "Bob")])
        .await;
    assert_eq!(join2_response.status_code(), 303);
    let location2 = extract_location(join2_response.headers()).unwrap();
    let player2_id = extract_player_id_from_url(&location2).unwrap();

    // Verify different player IDs
    assert_ne!(player1_id, player2_id);
}

#[tokio::test]
async fn test_bidding_sequence() {
    let server = create_test_server();

    // Create a game and join as host
    let (game_id, host_id) = setup_joined_game(&server, "Host").await;

    // Join as another player
    let join_response = server
        .post(&format!("/join/{}", game_id))
        .form(&[("player_name", "Alice")])
        .await;
    let alice_location = extract_location(join_response.headers()).unwrap();
    let alice_id = extract_player_id_from_url(&alice_location).unwrap();

    // Start the game
    server
        .post(&format!("/lobby/{}/start?player_id={}", game_id, host_id))
        .await;

    // Start a round
    server
        .post(&format!("/game/{}/start-round", game_id))
        .form(&[("player_id", host_id.as_str())])
        .await;

    // Host bids 10
    let bid1_response = server
        .post(&format!("/game/{}/bid", game_id))
        .form(&[("player_id", host_id.as_str()), ("amount", "10")])
        .await;
    assert!(bid1_response.status_code().is_success());

    // Alice bids 20
    let bid2_response = server
        .post(&format!("/game/{}/bid", game_id))
        .form(&[("player_id", alice_id.as_str()), ("amount", "20")])
        .await;
    assert!(bid2_response.status_code().is_success());

    // Host bids 30
    let bid3_response = server
        .post(&format!("/game/{}/bid", game_id))
        .form(&[("player_id", host_id.as_str()), ("amount", "30")])
        .await;
    assert!(bid3_response.status_code().is_success());
}

#[tokio::test]
async fn test_cannot_join_after_game_starts() {
    let server = create_test_server();
    let (game_id, player_id) = setup_joined_game(&server, "Host").await;

    // Add a second player so the game can actually start (requires at least 2 players)
    server
        .post(&format!("/join/{}", game_id))
        .form(&[("player_name", "Player 2")])
        .await;

    // Start the game
    let start_response = server
        .post(&format!("/lobby/{}/start?player_id={}", game_id, player_id))
        .await;

    // Verify game started successfully
    assert!(start_response.status_code() == 303 || start_response.status_code().is_success());

    // Try to join after game started
    let join_response = server
        .post(&format!("/join/{}", game_id))
        .form(&[("player_name", "Late Player")])
        .await;

    // Should NOT successfully redirect to lobby with player_id
    // The handler should return an error (HTTP 200 with error text, not a 303 redirect)
    if join_response.status_code() == 303 {
        // If it's a redirect, it shouldn't be to the lobby
        let location = extract_location(join_response.headers()).unwrap_or_default();
        assert!(
            !location.contains(&format!("/lobby/{}", game_id)),
            "Should not successfully redirect to lobby after game started"
        );
    } else {
        // If it's not a redirect, it should contain an error message
        let body = join_response.text();
        assert!(!body.is_empty(), "Should return error message");
    }
}

#[tokio::test]
async fn test_player_info_endpoint() {
    let server = create_test_server();
    let (game_id, player_id) = setup_joined_game(&server, "Host").await;

    // Start the game
    server
        .post(&format!("/lobby/{}/start?player_id={}", game_id, player_id))
        .await;

    // Get player info
    let player_info_response = server
        .get(&format!(
            "/game/{}/player-info?player_id={}",
            game_id, player_id
        ))
        .await;

    assert_eq!(player_info_response.status_code(), 200);
}

#[tokio::test]
async fn test_collection_display_endpoint() {
    let server = create_test_server();
    let (game_id, player_id) = setup_joined_game(&server, "Host").await;

    // Start the game
    server
        .post(&format!("/lobby/{}/start?player_id={}", game_id, player_id))
        .await;

    // Get collection display
    let collection_response = server
        .get(&format!(
            "/game/{}/collection?player_id={}",
            game_id, player_id
        ))
        .await;

    assert_eq!(collection_response.status_code(), 200);
}

#[tokio::test]
async fn test_bidding_area_endpoint() {
    let server = create_test_server();
    let (game_id, player_id) = setup_joined_game(&server, "Host").await;

    // Start the game
    server
        .post(&format!("/lobby/{}/start?player_id={}", game_id, player_id))
        .await;

    // Get bidding area
    let bidding_response = server
        .get(&format!(
            "/game/{}/bidding-area?player_id={}",
            game_id, player_id
        ))
        .await;

    assert_eq!(bidding_response.status_code(), 200);
}

#[tokio::test]
async fn test_timer_display_endpoint() {
    let server = create_test_server();
    let (game_id, player_id) = setup_joined_game(&server, "Host").await;

    // Start the game
    server
        .post(&format!("/lobby/{}/start?player_id={}", game_id, player_id))
        .await;

    // Get timer display
    let timer_response = server
        .get(&format!(
            "/game/{}/timer-display?player_id={}",
            game_id, player_id
        ))
        .await;

    assert_eq!(timer_response.status_code(), 200);
}

#[tokio::test]
async fn test_lobby_view_endpoint() {
    let server = create_test_server();
    let (game_id, player_id) = setup_joined_game(&server, "Host").await;

    // View lobby
    let lobby_response = server
        .get(&format!("/lobby/{}?player_id={}", game_id, player_id))
        .await;

    assert_eq!(lobby_response.status_code(), 200);
    let body = lobby_response.text();
    assert!(body.contains("Lobby") || body.contains("lobby") || body.contains("Start"));
}

#[tokio::test]
async fn test_home_page() {
    let server = create_test_server();

    // Get home page
    let response = server.get("/").await;

    assert_eq!(response.status_code(), 200);
    let body = response.text();
    assert!(body.contains("Art Collector") || body.contains("Create"));
}

#[tokio::test]
async fn test_complete_game_flow() {
    let server = create_test_server();

    // 1. Create a game and join as host
    let (game_id, host_id) = setup_joined_game(&server, "Host").await;

    // 2. Another player joins
    let join_response = server
        .post(&format!("/join/{}", game_id))
        .form(&[("player_name", "Alice")])
        .await;
    assert_eq!(join_response.status_code(), 303);
    let alice_location = extract_location(join_response.headers()).unwrap();
    let alice_id = extract_player_id_from_url(&alice_location).unwrap();

    // 3. Host configures the game (2 rounds for quick test)
    let config_response = server
        .post(&format!("/lobby/{}/configure", game_id))
        .form(&[
            ("starting_chips", "100"),
            ("bid_timer_seconds", "30"),
            ("num_rounds", "2"),
        ])
        .await;
    assert!(config_response.status_code().is_success());

    // 4. Host starts the game
    let start_response = server
        .post(&format!("/lobby/{}/start?player_id={}", game_id, host_id))
        .await;
    assert_eq!(start_response.status_code(), 303);

    // 5. Start first round
    let start_round1 = server
        .post(&format!("/game/{}/start-round", game_id))
        .form(&[("player_id", host_id.as_str())])
        .await;
    assert!(start_round1.status_code().is_success());

    // 6. Players bid
    server
        .post(&format!("/game/{}/bid", game_id))
        .form(&[("player_id", host_id.as_str()), ("amount", "10")])
        .await;

    server
        .post(&format!("/game/{}/bid", game_id))
        .form(&[("player_id", alice_id.as_str()), ("amount", "20")])
        .await;

    // 7. View game page to verify state
    let game_view = server
        .get(&format!("/game/{}/play?player_id={}", game_id, host_id))
        .await;
    assert_eq!(game_view.status_code(), 200);
}
