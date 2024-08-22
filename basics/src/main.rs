use rand::prelude::*;
mod basics;
use basics::enums::enum_examples;
use basics::structs::structs_example;
use basics::lifetimes::lifetimes_examples;
#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

/// inhererent implementation
impl Deck {
    // associated function
    fn new() -> Self {
        // Can also return Deck
        let mut deck = Deck { cards: Vec::new() };
        let suits = vec!["Hearts", "Spades", "Diamonds"];
        let values = vec!["Ace", "Two", "Three", "Four", "Five"];
        for suit in &suits {
            for value in &values {
                let card = format!("{} of {}", value, suit);
                println!("{card}");
                deck.cards.push(card)
            }
        }
        deck
    }
    // method
    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }
}

fn main() {
    println!("Hello, world!");
    let suits = vec!["Hearts", "Spades", "Diamonds"];
    let values = vec!["Ace", "Two", "Three", "Four", "Five"];
    let mut deck = Deck { cards: vec![] };
    println!("{deck:#?}");
    for suit in &suits {
        for value in &values {
            let card = format!("{} of {}", value, suit);
            println!("{card}");
            deck.cards.push(card)
        }
    }

    println!("{:#?}", deck);

    let mut d = Deck::new();
    println!("{:#?}", d);

    d.shuffle();
    println!("{:#?}", d);
    enum_examples();
    lifetimes_examples();
    structs_example();
    
}
