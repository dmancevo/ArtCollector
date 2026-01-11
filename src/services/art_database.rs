use crate::models::{ArtPiece, Artist, Movement};
use once_cell::sync::Lazy;
use rand::seq::SliceRandom;

/// Database of 90 art pieces: 30 artists × 3 paintings, 10 movements × 9 paintings
pub static ART_PIECES: Lazy<Vec<ArtPiece>> = Lazy::new(|| {
    vec![
        // Van Gogh - Post-Impressionism (3 pieces)
        ArtPiece {
            id: 1,
            name: "The Starry Night",
            artist: Artist::VanGogh,
            movement: Movement::PostImpressionism,
            stars: 3,
        },
        ArtPiece {
            id: 2,
            name: "Sunflowers",
            artist: Artist::VanGogh,
            movement: Movement::PostImpressionism,
            stars: 2,
        },
        ArtPiece {
            id: 3,
            name: "The Bedroom",
            artist: Artist::VanGogh,
            movement: Movement::PostImpressionism,
            stars: 2,
        },
        // Monet - Impressionism (3 pieces)
        ArtPiece {
            id: 4,
            name: "Water Lilies",
            artist: Artist::Monet,
            movement: Movement::Impressionism,
            stars: 3,
        },
        ArtPiece {
            id: 5,
            name: "Impression, Sunrise",
            artist: Artist::Monet,
            movement: Movement::Impressionism,
            stars: 3,
        },
        ArtPiece {
            id: 6,
            name: "Woman with a Parasol",
            artist: Artist::Monet,
            movement: Movement::Impressionism,
            stars: 2,
        },
        // Renoir - Impressionism (3 pieces)
        ArtPiece {
            id: 7,
            name: "Dance at Le Moulin de la Galette",
            artist: Artist::Renoir,
            movement: Movement::Impressionism,
            stars: 3,
        },
        ArtPiece {
            id: 8,
            name: "Luncheon of the Boating Party",
            artist: Artist::Renoir,
            movement: Movement::Impressionism,
            stars: 2,
        },
        ArtPiece {
            id: 9,
            name: "Girl with a Hoop",
            artist: Artist::Renoir,
            movement: Movement::Impressionism,
            stars: 1,
        },
        // Picasso - Cubism (3 pieces)
        ArtPiece {
            id: 10,
            name: "Guernica",
            artist: Artist::Picasso,
            movement: Movement::Cubism,
            stars: 3,
        },
        ArtPiece {
            id: 11,
            name: "Les Demoiselles d'Avignon",
            artist: Artist::Picasso,
            movement: Movement::Cubism,
            stars: 3,
        },
        ArtPiece {
            id: 12,
            name: "The Weeping Woman",
            artist: Artist::Picasso,
            movement: Movement::Cubism,
            stars: 2,
        },
        // Dali - Surrealism (3 pieces)
        ArtPiece {
            id: 13,
            name: "The Persistence of Memory",
            artist: Artist::Dali,
            movement: Movement::Surrealism,
            stars: 3,
        },
        ArtPiece {
            id: 14,
            name: "The Elephants",
            artist: Artist::Dali,
            movement: Movement::Surrealism,
            stars: 2,
        },
        ArtPiece {
            id: 15,
            name: "Swans Reflecting Elephants",
            artist: Artist::Dali,
            movement: Movement::Surrealism,
            stars: 2,
        },
        // Warhol - Pop Art (3 pieces)
        ArtPiece {
            id: 16,
            name: "Campbell's Soup Cans",
            artist: Artist::Warhol,
            movement: Movement::PopArt,
            stars: 3,
        },
        ArtPiece {
            id: 17,
            name: "Marilyn Diptych",
            artist: Artist::Warhol,
            movement: Movement::PopArt,
            stars: 3,
        },
        ArtPiece {
            id: 18,
            name: "Eight Elvises",
            artist: Artist::Warhol,
            movement: Movement::PopArt,
            stars: 2,
        },
        // Klimt - Art Nouveau (3 pieces)
        ArtPiece {
            id: 19,
            name: "The Kiss",
            artist: Artist::Klimt,
            movement: Movement::ArtNouveau,
            stars: 3,
        },
        ArtPiece {
            id: 20,
            name: "Portrait of Adele Bloch-Bauer I",
            artist: Artist::Klimt,
            movement: Movement::ArtNouveau,
            stars: 3,
        },
        ArtPiece {
            id: 21,
            name: "The Tree of Life",
            artist: Artist::Klimt,
            movement: Movement::ArtNouveau,
            stars: 2,
        },
        // Rembrandt - Baroque (3 pieces)
        ArtPiece {
            id: 22,
            name: "The Night Watch",
            artist: Artist::Rembrandt,
            movement: Movement::Baroque,
            stars: 3,
        },
        ArtPiece {
            id: 23,
            name: "Self-Portrait with Two Circles",
            artist: Artist::Rembrandt,
            movement: Movement::Baroque,
            stars: 2,
        },
        ArtPiece {
            id: 24,
            name: "The Anatomy Lesson",
            artist: Artist::Rembrandt,
            movement: Movement::Baroque,
            stars: 2,
        },
        // Da Vinci - Renaissance (3 pieces)
        ArtPiece {
            id: 25,
            name: "Mona Lisa",
            artist: Artist::DaVinci,
            movement: Movement::Renaissance,
            stars: 3,
        },
        ArtPiece {
            id: 26,
            name: "The Last Supper",
            artist: Artist::DaVinci,
            movement: Movement::Renaissance,
            stars: 3,
        },
        ArtPiece {
            id: 27,
            name: "Vitruvian Man",
            artist: Artist::DaVinci,
            movement: Movement::Renaissance,
            stars: 2,
        },
        // Michelangelo - Renaissance (3 pieces)
        ArtPiece {
            id: 28,
            name: "The Creation of Adam",
            artist: Artist::Michelangelo,
            movement: Movement::Renaissance,
            stars: 3,
        },
        ArtPiece {
            id: 29,
            name: "David",
            artist: Artist::Michelangelo,
            movement: Movement::Renaissance,
            stars: 3,
        },
        ArtPiece {
            id: 30,
            name: "The Last Judgment",
            artist: Artist::Michelangelo,
            movement: Movement::Renaissance,
            stars: 2,
        },
        // Raphael - Renaissance (3 pieces)
        ArtPiece {
            id: 31,
            name: "The School of Athens",
            artist: Artist::Raphael,
            movement: Movement::Renaissance,
            stars: 3,
        },
        ArtPiece {
            id: 32,
            name: "The Sistine Madonna",
            artist: Artist::Raphael,
            movement: Movement::Renaissance,
            stars: 2,
        },
        ArtPiece {
            id: 33,
            name: "The Transfiguration",
            artist: Artist::Raphael,
            movement: Movement::Renaissance,
            stars: 2,
        },
        // Caravaggio - Baroque (3 pieces)
        ArtPiece {
            id: 34,
            name: "The Calling of St Matthew",
            artist: Artist::Caravaggio,
            movement: Movement::Baroque,
            stars: 3,
        },
        ArtPiece {
            id: 35,
            name: "Judith Beheading Holofernes",
            artist: Artist::Caravaggio,
            movement: Movement::Baroque,
            stars: 2,
        },
        ArtPiece {
            id: 36,
            name: "The Conversion of St Paul",
            artist: Artist::Caravaggio,
            movement: Movement::Baroque,
            stars: 2,
        },
        // Vermeer - Baroque (3 pieces)
        ArtPiece {
            id: 37,
            name: "Girl with a Pearl Earring",
            artist: Artist::Vermeer,
            movement: Movement::Baroque,
            stars: 3,
        },
        ArtPiece {
            id: 38,
            name: "The Milkmaid",
            artist: Artist::Vermeer,
            movement: Movement::Baroque,
            stars: 2,
        },
        ArtPiece {
            id: 39,
            name: "View of Delft",
            artist: Artist::Vermeer,
            movement: Movement::Baroque,
            stars: 1,
        },
        // Frida Kahlo - Surrealism (3 pieces)
        ArtPiece {
            id: 40,
            name: "The Two Fridas",
            artist: Artist::FridaKahlo,
            movement: Movement::Surrealism,
            stars: 3,
        },
        ArtPiece {
            id: 41,
            name: "Self-Portrait with Thorn Necklace",
            artist: Artist::FridaKahlo,
            movement: Movement::Surrealism,
            stars: 2,
        },
        ArtPiece {
            id: 42,
            name: "The Broken Column",
            artist: Artist::FridaKahlo,
            movement: Movement::Surrealism,
            stars: 2,
        },
        // Matisse - PostImpressionism (3 pieces)
        ArtPiece {
            id: 43,
            name: "The Dance",
            artist: Artist::Matisse,
            movement: Movement::PostImpressionism,
            stars: 3,
        },
        ArtPiece {
            id: 44,
            name: "Blue Nude",
            artist: Artist::Matisse,
            movement: Movement::PostImpressionism,
            stars: 2,
        },
        ArtPiece {
            id: 45,
            name: "The Red Studio",
            artist: Artist::Matisse,
            movement: Movement::PostImpressionism,
            stars: 2,
        },
        // Cezanne - PostImpressionism (3 pieces)
        ArtPiece {
            id: 46,
            name: "The Card Players",
            artist: Artist::Cezanne,
            movement: Movement::PostImpressionism,
            stars: 3,
        },
        ArtPiece {
            id: 47,
            name: "Mont Sainte-Victoire",
            artist: Artist::Cezanne,
            movement: Movement::PostImpressionism,
            stars: 2,
        },
        ArtPiece {
            id: 48,
            name: "The Bathers",
            artist: Artist::Cezanne,
            movement: Movement::PostImpressionism,
            stars: 2,
        },
        // Gauguin - PostImpressionism (3 pieces)
        ArtPiece {
            id: 49,
            name: "Where Do We Come From?",
            artist: Artist::Gauguin,
            movement: Movement::PostImpressionism,
            stars: 3,
        },
        ArtPiece {
            id: 50,
            name: "The Yellow Christ",
            artist: Artist::Gauguin,
            movement: Movement::PostImpressionism,
            stars: 2,
        },
        ArtPiece {
            id: 51,
            name: "Tahitian Women on the Beach",
            artist: Artist::Gauguin,
            movement: Movement::PostImpressionism,
            stars: 1,
        },
        // Seurat - PostImpressionism (3 pieces)
        ArtPiece {
            id: 52,
            name: "A Sunday on La Grande Jatte",
            artist: Artist::Seurat,
            movement: Movement::PostImpressionism,
            stars: 3,
        },
        ArtPiece {
            id: 53,
            name: "Bathers at Asnières",
            artist: Artist::Seurat,
            movement: Movement::PostImpressionism,
            stars: 2,
        },
        ArtPiece {
            id: 54,
            name: "The Circus",
            artist: Artist::Seurat,
            movement: Movement::PostImpressionism,
            stars: 1,
        },
        // Kandinsky - Abstract Expressionism (3 pieces)
        ArtPiece {
            id: 55,
            name: "Composition VIII",
            artist: Artist::Kandinsky,
            movement: Movement::AbstractExpressionism,
            stars: 3,
        },
        ArtPiece {
            id: 56,
            name: "Yellow-Red-Blue",
            artist: Artist::Kandinsky,
            movement: Movement::AbstractExpressionism,
            stars: 2,
        },
        ArtPiece {
            id: 57,
            name: "Squares with Concentric Circles",
            artist: Artist::Kandinsky,
            movement: Movement::AbstractExpressionism,
            stars: 2,
        },
        // Mondrian - Abstract Expressionism (3 pieces)
        ArtPiece {
            id: 58,
            name: "Composition with Red, Blue and Yellow",
            artist: Artist::Mondrian,
            movement: Movement::AbstractExpressionism,
            stars: 3,
        },
        ArtPiece {
            id: 59,
            name: "Broadway Boogie Woogie",
            artist: Artist::Mondrian,
            movement: Movement::AbstractExpressionism,
            stars: 2,
        },
        ArtPiece {
            id: 60,
            name: "Victory Boogie Woogie",
            artist: Artist::Mondrian,
            movement: Movement::AbstractExpressionism,
            stars: 1,
        },
        // Pollock - Abstract Expressionism (3 pieces)
        ArtPiece {
            id: 61,
            name: "No. 5, 1948",
            artist: Artist::Pollock,
            movement: Movement::AbstractExpressionism,
            stars: 3,
        },
        ArtPiece {
            id: 62,
            name: "Blue Poles",
            artist: Artist::Pollock,
            movement: Movement::AbstractExpressionism,
            stars: 2,
        },
        ArtPiece {
            id: 63,
            name: "Autumn Rhythm",
            artist: Artist::Pollock,
            movement: Movement::AbstractExpressionism,
            stars: 2,
        },
        // Rothko - Abstract Expressionism (3 pieces)
        ArtPiece {
            id: 64,
            name: "Orange, Red, Yellow",
            artist: Artist::Rothko,
            movement: Movement::AbstractExpressionism,
            stars: 3,
        },
        ArtPiece {
            id: 65,
            name: "No. 61 (Rust and Blue)",
            artist: Artist::Rothko,
            movement: Movement::AbstractExpressionism,
            stars: 2,
        },
        ArtPiece {
            id: 66,
            name: "White Center",
            artist: Artist::Rothko,
            movement: Movement::AbstractExpressionism,
            stars: 1,
        },
        // Basquiat - Pop Art (3 pieces)
        ArtPiece {
            id: 67,
            name: "Untitled (1982)",
            artist: Artist::Basquiat,
            movement: Movement::PopArt,
            stars: 3,
        },
        ArtPiece {
            id: 68,
            name: "Hollywood Africans",
            artist: Artist::Basquiat,
            movement: Movement::PopArt,
            stars: 2,
        },
        ArtPiece {
            id: 69,
            name: "Irony of Negro Policeman",
            artist: Artist::Basquiat,
            movement: Movement::PopArt,
            stars: 1,
        },
        // Hopper - Expressionism (3 pieces)
        ArtPiece {
            id: 70,
            name: "Nighthawks",
            artist: Artist::Hopper,
            movement: Movement::Expressionism,
            stars: 3,
        },
        ArtPiece {
            id: 71,
            name: "Automat",
            artist: Artist::Hopper,
            movement: Movement::Expressionism,
            stars: 2,
        },
        ArtPiece {
            id: 72,
            name: "Morning Sun",
            artist: Artist::Hopper,
            movement: Movement::Expressionism,
            stars: 1,
        },
        // Munch - Expressionism (3 pieces)
        ArtPiece {
            id: 73,
            name: "The Scream",
            artist: Artist::Munch,
            movement: Movement::Expressionism,
            stars: 3,
        },
        ArtPiece {
            id: 74,
            name: "The Madonna",
            artist: Artist::Munch,
            movement: Movement::Expressionism,
            stars: 2,
        },
        ArtPiece {
            id: 75,
            name: "The Sick Child",
            artist: Artist::Munch,
            movement: Movement::Expressionism,
            stars: 2,
        },
        // Bruegel - Renaissance (3 pieces)
        ArtPiece {
            id: 76,
            name: "The Tower of Babel",
            artist: Artist::Bruegel,
            movement: Movement::Renaissance,
            stars: 3,
        },
        ArtPiece {
            id: 77,
            name: "The Hunters in the Snow",
            artist: Artist::Bruegel,
            movement: Movement::Renaissance,
            stars: 2,
        },
        ArtPiece {
            id: 78,
            name: "Netherlandish Proverbs",
            artist: Artist::Bruegel,
            movement: Movement::Renaissance,
            stars: 1,
        },
        // Bosch - Renaissance (3 pieces)
        ArtPiece {
            id: 79,
            name: "The Garden of Earthly Delights",
            artist: Artist::Bosch,
            movement: Movement::Renaissance,
            stars: 3,
        },
        ArtPiece {
            id: 80,
            name: "The Haywain Triptych",
            artist: Artist::Bosch,
            movement: Movement::Renaissance,
            stars: 2,
        },
        ArtPiece {
            id: 81,
            name: "The Temptation of St. Anthony",
            artist: Artist::Bosch,
            movement: Movement::Renaissance,
            stars: 1,
        },
        // El Greco - Baroque (3 pieces)
        ArtPiece {
            id: 82,
            name: "The Burial of the Count of Orgaz",
            artist: Artist::ElGreco,
            movement: Movement::Baroque,
            stars: 3,
        },
        ArtPiece {
            id: 83,
            name: "View of Toledo",
            artist: Artist::ElGreco,
            movement: Movement::Baroque,
            stars: 2,
        },
        ArtPiece {
            id: 84,
            name: "The Disrobing of Christ",
            artist: Artist::ElGreco,
            movement: Movement::Baroque,
            stars: 1,
        },
        // Botticelli - Renaissance (3 pieces)
        ArtPiece {
            id: 85,
            name: "The Birth of Venus",
            artist: Artist::Botticelli,
            movement: Movement::Renaissance,
            stars: 3,
        },
        ArtPiece {
            id: 86,
            name: "Primavera",
            artist: Artist::Botticelli,
            movement: Movement::Renaissance,
            stars: 3,
        },
        ArtPiece {
            id: 87,
            name: "The Adoration of the Magi",
            artist: Artist::Botticelli,
            movement: Movement::Renaissance,
            stars: 1,
        },
        // Titian - Renaissance (3 pieces)
        ArtPiece {
            id: 88,
            name: "Assumption of the Virgin",
            artist: Artist::Titian,
            movement: Movement::Renaissance,
            stars: 3,
        },
        ArtPiece {
            id: 89,
            name: "Venus of Urbino",
            artist: Artist::Titian,
            movement: Movement::Renaissance,
            stars: 2,
        },
        ArtPiece {
            id: 90,
            name: "Bacchus and Ariadne",
            artist: Artist::Titian,
            movement: Movement::Renaissance,
            stars: 1,
        },
    ]
});

/// Returns a shuffled subset of paintings for a game
/// Size = 5 × number of players
#[allow(dead_code)]
pub fn get_game_deck(num_players: usize) -> Vec<ArtPiece> {
    let deck_size = (num_players * 5).min(90);
    get_game_deck_by_count(deck_size)
}

/// Returns a shuffled subset of paintings for a game with a specific count
pub fn get_game_deck_by_count(count: usize) -> Vec<ArtPiece> {
    let deck_size = count.clamp(1, 90); // At least 1, at most 90
    let mut rng = rand::thread_rng();

    let mut deck: Vec<ArtPiece> = ART_PIECES.clone();
    deck.shuffle(&mut rng);
    deck.truncate(deck_size);

    deck
}
