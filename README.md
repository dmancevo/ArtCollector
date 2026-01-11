# 🎨 Art Collector

A multiplayer real-time art auction game built with Rust, Axum, and HTMX. Players bid on famous masterpieces to build the most valuable art collection!

## 🎮 Game Overview

Art Collector is a strategic bidding game where players compete to collect paintings from 30 famous artists across 10 art movements. The winner is determined by collecting pieces that maximize bonuses for artists or movements.

### Key Features

- **Real-time multiplayer** using Server-Sent Events (SSE)
- **No client-side JavaScript** - pure server-rendered with HTMX
- **90 famous paintings** from masters like Van Gogh, Monet, Picasso, and more
- **Strategic scoring system** with artist and movement bonuses
- **Host-configurable settings** (starting chips, timer duration, number of rounds)
- **Manual round control** - host starts each bidding round
- **Play Again feature** - seamless rematch with same players

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- Cargo (comes with Rust)

### Running the Game

1. Clone the repository:
```bash
git clone <repository-url>
cd collector
```

2. Build and run:
```bash
cargo run
```

3. Open your browser to:
```
http://localhost:3000
```

4. Create a game, share the join link with friends, and start playing!

## 📖 How to Play

### Game Flow

1. **Create & Join**
   - Host creates a game and shares the join link
   - Players enter their names to join
   - Host configures game settings (chips, timer, rounds)

2. **Bidding Rounds**
   - Host manually starts each round
   - Players bid on the displayed artwork
   - Each bid resets the timer to full duration
   - Highest bid wins when timer expires

3. **Building Collections**
   - Winner adds the artwork to their collection
   - If nobody bids, the artwork is discarded
   - Players with 0 chips can watch but can't bid

4. **Final Results**
   - After all rounds, scores are calculated
   - Player with the highest score wins!

### Scoring System

Your final score is the **maximum** of your Artist Bonus or Movement Bonus:

**Artist Bonus**: For each artist you collect, calculate:
```
(sum of star values) × (number of pieces)
```

**Movement Bonus**: For each movement you collect, calculate:
```
(sum of star values) × (number of pieces)
```

**Final Score**: `MAX(Artist Bonus, Movement Bonus)`

#### Scoring Example

**Collection:**
- 3 Van Gogh paintings (3★, 2★, 2★)
- 1 Monet painting (3★)

**Artist Bonus:**
- Van Gogh: (3+2+2) × 3 = 21 points
- Monet: 3 × 1 = 3 points
- Total: 24 points

**Movement Bonus:**
- Post-Impressionism (Van Goghs): (3+2+2) × 3 = 21 points
- Impressionism (Monet): 3 × 1 = 3 points
- Total: 24 points

**Final Score**: MAX(24, 24) = **24 points**

💡 **Strategy Tip**: Collect multiple pieces from the same artist OR movement to maximize your multiplier!

## 🛠️ Tech Stack

- **Backend**: Rust with [Axum](https://github.com/tokio-rs/axum) web framework
- **Frontend**: HTML with [HTMX](https://htmx.org/) for dynamic updates
- **Styling**: [Tailwind CSS](https://tailwindcss.com/) + [DaisyUI](https://daisyui.com/)
- **Templates**: [Askama](https://github.com/djc/askama) (type-safe, compile-time)
- **Real-time**: Server-Sent Events (SSE) via HTMX
- **State Management**: In-memory with `Arc<RwLock<HashMap>>`
- **ID Generation**: [nanoid](https://github.com/nikolay-govorov/nanoid)

## 🎨 Art Database

The game features 90 carefully curated famous paintings:

- **30 Artists** (3 paintings each):
  - Renaissance masters: Da Vinci, Michelangelo, Raphael
  - Impressionists: Monet, Renoir, Degas
  - Post-Impressionists: Van Gogh, Cézanne, Gauguin
  - Modern artists: Picasso, Dalí, Warhol, Pollock
  - And many more!

- **10 Art Movements** (9 paintings each):
  - Renaissance
  - Baroque
  - Impressionism
  - Post-Impressionism
  - Cubism
  - Surrealism
  - Pop Art
  - Abstract Expressionism
  - Art Nouveau
  - Expressionism

## 🏗️ Project Structure

```
src/
├── main.rs                    # Server setup
├── routes.rs                  # Route configuration
├── models/
│   ├── game.rs               # Game state and logic
│   ├── player.rs             # Player with scoring
│   ├── art.rs                # Art database
│   └── bid.rs                # Bid model
├── handlers/
│   ├── home.rs               # Landing page
│   ├── lobby.rs              # Game lobby
│   ├── game.rs               # Active game
│   ├── results.rs            # Results page
│   ├── partials.rs           # HTML fragments
│   └── sse.rs                # SSE event stream
├── services/
│   ├── game_engine.rs        # Game logic
│   ├── timer.rs              # Round timer
│   └── art_database.rs       # Art data
├── state/
│   └── app_state.rs          # Shared state
└── templates/
    ├── base.html             # Base layout
    ├── home.html             # Landing page
    ├── lobby.html            # Lobby view
    ├── game.html             # Game view
    └── results.html          # Results page
```

## 🔧 Configuration

Game hosts can configure:

- **Starting Chips**: 50-500 (default: 100)
- **Bid Timer**: 15-60 seconds (default: 30s)
- **Number of Rounds**: 1-90 (default: 10)

## 🌐 Development

### Running in Development

```bash
cargo run
```

The server will start on `http://localhost:3000` with hot reload via `cargo watch`:

```bash
cargo install cargo-watch
cargo watch -x run
```

### Building for Production

```bash
cargo build --release
./target/release/collector
```

### Running Tests

```bash
cargo test
```

## 🎯 Game Design Decisions

1. **SSE over WebSockets**: HTMX has native SSE support, and unidirectional updates are sufficient for this game

2. **Server-Rendered**: No client-side JavaScript needed - all logic on the server, HTMX handles interactivity

3. **Manual Round Start**: Host controls pacing, preventing rushed gameplay

4. **Timer Resets on Bid**: Each bid extends the round, ensuring fair competition

5. **MAX Scoring**: Takes higher of artist/movement bonus to reward focused collecting strategies

6. **In-Memory State**: Simple deployment, perfect for party games (no database needed)

## 🤝 Contributing

Contributions are welcome! Feel free to:

- Report bugs
- Suggest new features
- Submit pull requests
- Add more artworks to the database

## 🎉 Credits

Built with ❤️ using Rust and modern web technologies. Special thanks to the Rust community and the creators of Axum, HTMX, and Askama.

---

**Enjoy collecting art! 🎨**
