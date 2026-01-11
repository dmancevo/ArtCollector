use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Artist {
    VanGogh,
    Monet,
    Renoir,
    Picasso,
    Dali,
    Warhol,
    Klimt,
    Rembrandt,
    DaVinci,
    Michelangelo,
    Raphael,
    Caravaggio,
    Vermeer,
    FridaKahlo,
    Matisse,
    Cezanne,
    Gauguin,
    Seurat,
    Kandinsky,
    Mondrian,
    Pollock,
    Rothko,
    Basquiat,
    Hopper,
    Munch,
    Bruegel,
    Bosch,
    ElGreco,
    Botticelli,
    Titian,
}

impl Artist {
    #[allow(dead_code)]
    pub fn name(&self) -> &'static str {
        match self {
            Artist::VanGogh => "Vincent van Gogh",
            Artist::Monet => "Claude Monet",
            Artist::Renoir => "Pierre-Auguste Renoir",
            Artist::Picasso => "Pablo Picasso",
            Artist::Dali => "Salvador Dalí",
            Artist::Warhol => "Andy Warhol",
            Artist::Klimt => "Gustav Klimt",
            Artist::Rembrandt => "Rembrandt van Rijn",
            Artist::DaVinci => "Leonardo da Vinci",
            Artist::Michelangelo => "Michelangelo Buonarroti",
            Artist::Raphael => "Raffaello Sanzio",
            Artist::Caravaggio => "Michelangelo Merisi da Caravaggio",
            Artist::Vermeer => "Johannes Vermeer",
            Artist::FridaKahlo => "Frida Kahlo",
            Artist::Matisse => "Henri Matisse",
            Artist::Cezanne => "Paul Cézanne",
            Artist::Gauguin => "Paul Gauguin",
            Artist::Seurat => "Georges Seurat",
            Artist::Kandinsky => "Wassily Kandinsky",
            Artist::Mondrian => "Piet Mondrian",
            Artist::Pollock => "Jackson Pollock",
            Artist::Rothko => "Mark Rothko",
            Artist::Basquiat => "Jean-Michel Basquiat",
            Artist::Hopper => "Edward Hopper",
            Artist::Munch => "Edvard Munch",
            Artist::Bruegel => "Pieter Bruegel the Elder",
            Artist::Bosch => "Hieronymus Bosch",
            Artist::ElGreco => "El Greco",
            Artist::Botticelli => "Sandro Botticelli",
            Artist::Titian => "Titian",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Movement {
    Renaissance,
    Baroque,
    Impressionism,
    PostImpressionism,
    Cubism,
    Surrealism,
    PopArt,
    AbstractExpressionism,
    ArtNouveau,
    Expressionism,
}

impl Movement {
    #[allow(dead_code)]
    pub fn name(&self) -> &'static str {
        match self {
            Movement::Renaissance => "Renaissance",
            Movement::Baroque => "Baroque",
            Movement::Impressionism => "Impressionism",
            Movement::PostImpressionism => "Post-Impressionism",
            Movement::Cubism => "Cubism",
            Movement::Surrealism => "Surrealism",
            Movement::PopArt => "Pop Art",
            Movement::AbstractExpressionism => "Abstract Expressionism",
            Movement::ArtNouveau => "Art Nouveau",
            Movement::Expressionism => "Expressionism",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ArtPiece {
    pub id: usize,
    pub name: &'static str,
    pub artist: Artist,
    pub movement: Movement,
    pub stars: u8, // 1-3
}
