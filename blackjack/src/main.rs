// Module import
use rand::seq::SliceRandom;
use rand::thread_rng;
use prompted::input;
use std::{collections::{HashMap, hash_map}, vec};

fn main() {

    let mut deck: Vec<&str> = Vec::from(["A", "K", "Q", "J", "10", "9", "8", "7", "6", "5", "4", "3", "2","A","K", "Q", "J", "10", "9", "8", "7", "6", "5", "4", "3","2","A","K", "Q", "J", "10", "9", "8", "7", "6", "5", "4", "3", "2","A","K", "Q", "J", "10", "9", "8", "7", "6", "5", "4", "3", "2"]);

    deck.shuffle(&mut thread_rng()); // Shuffle deck

    let mut dealer_hand: Vec<&str> = Vec::new();
    let mut player_hand: Vec<&str> = Vec::new();
    
    draw(&mut player_hand, deck);
    draw(&mut dealer_hand, deck);
    draw(&mut player_hand, deck);
    draw(&mut dealer_hand, deck);
    
    println!("{:?}", deck);
    println!("{:?}", player_hand);
    println!("{:?}", dealer_hand);

    loop {
        let action = input!("Would you like to hit (H) or stand (S)?");
        // Implement actions for cards

        if action == "H" {
            draw((&mut player_hand), deck);

        } else if action == "S" {
            break
        }
        
    }
}

fn calc_hand(hand: &[&str]) -> (u32, bool) {
    let mut value: i32 = 0;
    let mut aces: i32 = 0;

    let val_map = HashMap::from([
        ("A", 11),
        ("K", 10),
        ("Q", 10),
        ("J", 10),
        ("10", 10),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);

    for card in hand {
        if *card == "A" {
            aces += 1;
        }
        value += val_map[*card];
    }


    while value > 21 && aces > 0 {
        value -= 10;
        aces -= 1;
    }

 
    let is_soft = aces > 0;

    (value as u32, is_soft)
}

fn draw_card(deck: &mut Vec<&str>) -> &str {
    deck.pop().expect("deck is empty")
    // draw function, has error handling but in regular blackjack shouldn't be possible to finish the deck
}

fn draw(hand: &mut Vec<&str>, deck: &mut Deck) {
    let card = draw_card(deck);
    hand.push(card);
    // adds a drawn card to a hand
}

fn init() {
    deck = ["A","K", "Q", "J", "10", "9", "8", "7", "6", "5", "4", "3", "2","A","K", "Q", "J", "10", "9", "8", "7", "6", "5", "4", "3","2","A","K", "Q", "J", "10", "9", "8", "7", "6", "5", "4", "3", "2","A","K", "Q", "J", "10", "9", "8", "7", "6", "5", "4", "3", "2"];
    draw(&mut player_hand, deck);

}


