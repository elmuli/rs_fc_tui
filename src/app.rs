use serde::{Deserialize, Serialize};
use serde_json::{json, Result};
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize)]
pub struct Card{
    pub word: String,
    pub kana: String,
    pub translation: String,
}

#[derive(Deserialize)]
struct CardDeck {
    cards: Vec<Card>,
}

fn read_cards(path: &str) -> Vec<Card> {
    let file = File::open(path).expect("could not open file");
    let deck: CardDeck = serde_json::from_reader(file).expect("failed to parse json");
    deck.cards
}

pub fn get_card(index: usize) -> Card{
    let cards = read_cards("cards.json");
    let card = &cards[index];
    return card;
}

#[derive(Default)]
pub struct App{
    pub should_quit: bool,
    pub is_help: bool,
}
