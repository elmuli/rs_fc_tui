use serde::{Deserialize, Serialize};
use serde_json::{json, Result};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use rand::Rng;

#[derive(Serialize, Deserialize, Clone)]
pub struct Card{
    pub word: String,
    pub kana: String,
    pub translation: String,
}

#[derive(Deserialize)]
struct CardDeck {
    cards: Vec<Card>,
}

fn read_cards(path: &Path) -> Vec<Card> {
    let file = File::open(path).expect("could not open file");
    let deck: CardDeck = serde_json::from_reader(file).expect("failed to parse json");
    deck.cards
}

pub fn get_card(index: usize) -> Card{
    let path = Path::new("cards/cards.json");
    let cards = read_cards(path);
    let card = cards[index].clone();
    card
}

pub fn calculate_card_index() -> usize{
    let mut rng = rand::thread_rng();
    let n: usize = rng.gen_range(0..20);
    n
}

#[derive(Default)]
pub struct App{
    pub should_quit: bool,
    pub is_help: bool,
    pub card_index: usize,
}
