
#[derive(Debug)]
struct Deck {
    cards : Vec<String>
}

fn main() {
    println!("Hello, world!");
    let suits = vec!["Hearts", "Spades", "Diamonds"];
    let values = vec!["Ace", "Two", "Three", "Four", "Five"];
    let mut deck = Deck{ cards: vec![]};
    println!("{deck:#?}" );
    for suit in &suits{
        for value in &values{
            let card = format!("{} of {}", value, suit);
            println!("{card}");
            deck.cards.push(card)
        }
    }
    
    println!("{:#?}",deck );

}
