use crate::models::Game;

/// Render the timer display partial
pub fn render_timer_partial(game: &Game, player_id: &str, game_id: &str) -> String {
    let seconds = game.calculate_remaining_seconds();
    let is_host = game.is_host(player_id);

    if seconds < 0 {
        // Round hasn't started yet - show start button for host or waiting message for others
        let round = match &game.state {
            crate::models::GameState::Active { round, .. } => *round,
            _ => 0,
        };

        if is_host {
            format!(
                r#"<form
                    hx-post="/game/{}/start-round"
                    hx-swap="none"
                    class="flex flex-col gap-2 items-center"
                >
                    <input type="hidden" name="player_id" value="{}" />
                    <button type="submit" class="btn btn-primary btn-lg">
                        🎬 Start Round {}
                    </button>
                    <p class="text-xs opacity-50">Click to begin bidding</p>
                </form>"#,
                game_id, player_id, round
            )
        } else {
            r#"<div class="text-2xl font-bold text-warning">
                Waiting for host to start...
            </div>"#
                .to_string()
        }
    } else {
        format!(
            r#"<div class="text-4xl font-mono font-bold text-center {}">{}</div><p class="text-xs opacity-50 text-center">seconds remaining</p>"#,
            if seconds <= 5 {
                "text-error"
            } else {
                "text-info"
            },
            seconds
        )
    }
}

/// Render the current bid display partial
pub fn render_bid_partial(game: &Game) -> String {
    if let Some(bid) = game.get_highest_bid() {
        let player_name = game
            .players
            .get(&bid.player_id)
            .map(|p| p.name.as_str())
            .unwrap_or(&bid.player_id);

        format!(
            r#"<div class="alert alert-info">
                <div class="flex items-center gap-2">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                    <span>Current bid: <strong>{} chips</strong> by <strong>{}</strong></span>
                </div>
            </div>"#,
            bid.amount, player_name
        )
    } else {
        r#"<div class="alert alert-warning">
            <div class="flex items-center gap-2">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"></path></svg>
                <span>No bids yet - be the first to bid!</span>
            </div>
        </div>"#
            .to_string()
    }
}

/// Render the player list partial for lobby (matches template structure)
pub fn render_player_list_partial(game: &Game) -> String {
    let mut html = String::new();

    for player in game.players.values() {
        let is_host = game.is_host(&player.id);
        let first_char = player
            .name
            .chars()
            .next()
            .unwrap_or('?')
            .to_uppercase()
            .to_string();
        let host_badge = if is_host {
            r#"<div class="badge badge-primary">Host</div>"#
        } else {
            ""
        };

        html.push_str(&format!(
            r#"<div class="flex items-center gap-2 p-3 bg-base-200 rounded-lg">
                <div class="avatar placeholder">
                    <div class="bg-primary text-primary-content rounded-full w-10">
                        <span>{}</span>
                    </div>
                </div>
                <div class="flex-1">
                    <div class="font-semibold">{}</div>
                </div>
                {}
            </div>"#,
            first_char, player.name, host_badge
        ));
    }

    if game.players.is_empty() {
        html.push_str(
            r#"<div class="text-center py-8 opacity-50">
                <p>No players yet...</p>
            </div>"#,
        );
    }

    html
}

/// Render player info for game view (chips and collection count) - matches template structure
pub fn render_player_info_partial(
    game: &Game,
    player_id: &str,
    round: usize,
    player_name: &str,
) -> String {
    if let Some(player) = game.players.get(player_id) {
        format!(
            r#"<div class="stat bg-base-100 rounded-lg">
                <div class="stat-title">Your Chips</div>
                <div class="stat-value text-primary">{}</div>
            </div>
            <div>
                <h1 class="text-2xl font-bold">Round {}</h1>
                <p class="text-sm opacity-70">{}</p>
            </div>
            <div class="stat bg-base-100 rounded-lg">
                <div class="stat-title">Collection</div>
                <div class="stat-value text-secondary">{}</div>
                <div class="stat-desc">Score: {}</div>
            </div>"#,
            player.chips,
            round,
            player_name,
            player.collection.len(),
            player.calculate_score()
        )
    } else {
        r#"<div class="alert alert-error">Player not found</div>"#.to_string()
    }
}

/// Render the current art piece display - matches template structure
pub fn render_current_art_partial(game: &Game) -> String {
    if let Some(art) = &game.current_art {
        let stars: String = (0..art.stars).map(|_| "⭐").collect();

        format!(
            r#"<div>
                <h2 class="card-title text-2xl">{}</h2>
                <p class="text-lg opacity-70">{}</p>
                <p class="text-sm opacity-50">{}</p>
            </div>
            <div class="text-right">
                <div class="text-3xl">
                    {}
                </div>
                <p class="text-xs opacity-50">Value</p>
            </div>"#,
            art.name,
            art.artist.name(),
            art.movement.name(),
            stars
        )
    } else {
        r#"<div class="alert alert-info">No art piece available</div>"#.to_string()
    }
}

/// Render the collection display for the sidebar
pub fn render_collection_display_partial(player: &crate::models::Player) -> String {
    let collection_count = player.collection.len();
    let collection_score = player.calculate_score();

    if collection_count > 0 {
        let mut html = String::from(r#"<div class="space-y-2 max-h-96 overflow-y-auto">"#);

        // Add each art piece
        for art in &player.collection {
            let stars: String = (0..art.stars).map(|_| "⭐").collect();
            html.push_str(&format!(
                r#"<div class="card bg-base-200 shadow-sm">
                    <div class="card-body p-3">
                        <h4 class="font-semibold text-sm">{}</h4>
                        <div class="text-xs opacity-70 space-y-1">
                            <div>{}</div>
                            <div>🎨 {}</div>
                            <div>🏛️ {}</div>
                        </div>
                    </div>
                </div>"#,
                art.name,
                stars,
                art.artist.name(),
                art.movement.name()
            ));
        }

        // Add score summary at the bottom
        html.push_str(&format!(
            r#"<div class="stat bg-base-300 rounded-lg mt-2">
                <div class="stat-title">Total Score</div>
                <div class="stat-value text-sm text-primary">{}</div>
                <div class="stat-desc">With bonuses</div>
            </div>
        </div>"#,
            collection_score
        ));

        html
    } else {
        r#"<p class="text-center opacity-50 py-8">No pieces yet</p>"#.to_string()
    }
}

/// Render the bidding form area
pub fn render_bidding_area_partial(game: &Game, player_id: &str, game_id: &str) -> String {
    let timer_seconds = game.calculate_remaining_seconds();

    let player = match game.players.get(player_id) {
        Some(p) => p,
        None => return r#"<div class="alert alert-error">Player not found</div>"#.to_string(),
    };

    let player_chips = player.chips;

    if timer_seconds < 0 {
        return r#"<div class="alert alert-info">
            <span>⏸️ Waiting for host to start bidding...</span>
        </div>"#
            .to_string();
    }

    if player_chips <= 0 {
        return r#"<div class="alert alert-warning">
            <span>You have no chips left! You can only watch.</span>
        </div>"#
            .to_string();
    }

    let highest_bid = game.get_highest_bid();
    let has_bid = highest_bid.is_some();
    let current_highest_bid = highest_bid.as_ref().map(|b| b.amount).unwrap_or(0);
    let min_bid = if has_bid { current_highest_bid + 1 } else { 1 };

    format!(
        r#"<form
            hx-post="/game/{}/bid"
            hx-swap="none"
            class="flex gap-2"
        >
            <input type="hidden" name="player_id" value="{}" />
            <input
                type="number"
                name="amount"
                placeholder="Enter bid amount"
                class="input input-bordered flex-1"
                min="{}"
                max="{}"
                required
            />
            <button type="submit" class="btn btn-primary">
                Place Bid
            </button>
            <button
                type="button"
                class="btn btn-ghost"
                hx-post="/game/{}/pass"
                hx-vals='{{"player_id": "{}"}}'
                hx-swap="none"
            >
                Pass
            </button>
        </form>
        <div class="flex gap-2 mt-2">
            <button
                type="button"
                class="btn btn-sm btn-outline"
                onclick="var input = document.querySelector('input[name=amount]'); input.value = {};"
            >
                Min ({})
            </button>
            <button
                type="button"
                class="btn btn-sm btn-outline"
                onclick="var input = document.querySelector('input[name=amount]'); var current = parseInt(input.value) || 0; input.value = Math.min(Math.max(current + 1, {}), {});"
            >
                +1
            </button>
            <button
                type="button"
                class="btn btn-sm btn-outline"
                onclick="var input = document.querySelector('input[name=amount]'); var current = parseInt(input.value) || 0; input.value = Math.min(Math.max(current + 5, {}), {});"
            >
                +5
            </button>
            <button
                type="button"
                class="btn btn-sm btn-outline"
                onclick="var input = document.querySelector('input[name=amount]'); var current = parseInt(input.value) || 0; input.value = Math.min(Math.max(current + 10, {}), {});"
            >
                +10
            </button>
        </div>"#,
        game_id,
        player_id,
        min_bid,
        player_chips,
        game_id,
        player_id,
        min_bid,
        min_bid,
        min_bid,
        player_chips,
        min_bid,
        player_chips,
        min_bid,
        player_chips
    )
}

/// Render a simple trigger message for game-started
/// The actual redirect is handled by the page's hidden trigger element
pub fn render_game_started_trigger() -> String {
    // Just send any content - the SSE event name "game-started" is what matters
    // The page has a hidden div with hx-trigger="sse:game-started" that will handle the redirect
    String::from("<!-- Game starting -->")
}

/// Render a simple trigger message for game-finished
pub fn render_game_finished_trigger() -> String {
    // Just send any content - the SSE event name "game-finished" is what matters
    String::from("<!-- Game finished -->")
}

/// Render the complete players card for lobby (with count)
pub fn render_lobby_players_card(game: &Game) -> String {
    let player_count = game.players.len();

    format!(
        r#"<h2 class="card-title">Players ({})</h2>
<div class="space-y-2">
{}
</div>"#,
        player_count,
        render_player_list_partial(game)
    )
}

/// Render the game settings section for host
pub fn render_lobby_settings_host(game: &Game, game_id: &str, player_id: &str) -> String {
    let player_count = game.players.len();
    let can_start = player_count >= 2;
    let button_text = if can_start {
        "Start Game"
    } else {
        "Waiting for players..."
    };
    let disabled = if can_start { "" } else { "disabled" };

    format!(
        r#"<h2 class="card-title">Game Settings</h2>
<form
    hx-post="/lobby/{}/configure"
    hx-swap="none"
    hx-trigger="change"
    class="space-y-4"
>
    <div class="form-control">
        <label class="label">
            <span class="label-text">Starting Chips</span>
            <span class="label-text-alt" id="chips-value">{}</span>
        </label>
        <input
            type="range"
            name="starting_chips"
            min="50"
            max="500"
            value="{}"
            class="range range-primary"
            step="50"
            oninput="document.getElementById('chips-value').textContent = this.value"
        />
        <div class="w-full flex justify-between text-xs px-2 opacity-50">
            <span>50</span>
            <span>500</span>
        </div>
    </div>

    <div class="form-control">
        <label class="label">
            <span class="label-text">Bid Timer (seconds)</span>
            <span class="label-text-alt" id="timer-value">{}</span>
        </label>
        <input
            type="range"
            name="bid_timer_seconds"
            min="15"
            max="60"
            value="{}"
            class="range range-secondary"
            step="5"
            oninput="document.getElementById('timer-value').textContent = this.value"
        />
        <div class="w-full flex justify-between text-xs px-2 opacity-50">
            <span>15s</span>
            <span>60s</span>
        </div>
    </div>

    <div class="form-control">
        <label class="label">
            <span class="label-text">Number of Rounds</span>
            <span class="label-text-alt" id="rounds-value">{}</span>
        </label>
        <input
            type="range"
            name="num_rounds"
            min="1"
            max="90"
            value="{}"
            class="range range-accent"
            step="1"
            oninput="document.getElementById('rounds-value').textContent = this.value"
        />
        <div class="w-full flex justify-between text-xs px-2 opacity-50">
            <span>1</span>
            <span>90</span>
        </div>
    </div>
</form>

<div class="divider"></div>

<form
    action="/lobby/{}/start?player_id={}"
    method="post"
>
    <button
        type="submit"
        class="btn btn-primary btn-lg w-full"
        {}
    >
        {}
    </button>
</form>"#,
        game_id,
        game.config.starting_chips,
        game.config.starting_chips,
        game.config.bid_timer_seconds,
        game.config.bid_timer_seconds,
        game.config.num_rounds,
        game.config.num_rounds,
        game_id,
        player_id,
        disabled,
        button_text
    )
}

/// Render the game settings section for non-host players
pub fn render_lobby_settings_player(game: &Game) -> String {
    format!(
        r#"<h2 class="card-title">Game Settings</h2>
<div class="space-y-4">
    <div class="stat bg-base-200 rounded-lg">
        <div class="stat-title">Starting Chips</div>
        <div class="stat-value text-primary">{}</div>
    </div>

    <div class="stat bg-base-200 rounded-lg">
        <div class="stat-title">Bid Timer</div>
        <div class="stat-value text-secondary">{}s</div>
    </div>

    <div class="stat bg-base-200 rounded-lg">
        <div class="stat-title">Number of Rounds</div>
        <div class="stat-value text-accent">{}</div>
    </div>

    <div class="alert">
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-info shrink-0 w-6 h-6">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
        </svg>
        <span>Waiting for host to start the game...</span>
    </div>
</div>"#,
        game.config.starting_chips, game.config.bid_timer_seconds, game.config.num_rounds
    )
}
