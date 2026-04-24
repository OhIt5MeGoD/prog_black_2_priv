// Module import
use rand::prelude::*;
use prompted::input;
use std::{collections::{HashMap, hash_map}, vec};

fn main() {

    let mut rng = rand::rng();

    let mut deck: [&str; 52] = ["A","K", "Q", "J", "10", "9", "8", "7", "6", "5", "4", "3", "2","A","K", "Q", "J", "10", "9", "8", "7", "6", "5", "4", "3","2","A","K", "Q", "J", "10", "9", "8", "7", "6", "5", "4", "3", "2","A","K", "Q", "J", "10", "9", "8", "7", "6", "5", "4", "3", "2"];

    let mut dealer_hand = Vec::new();
    let mut player_hand = Vec::new();

    deck.shuffle(&mut rng);

    println!("{:?}", deck);

    loop {
        let action = input!("Would you like to hit (H) or stand (S)?");
        // Implement actions for cards

        if action == "H" {
            break
        } else if action == "S" {
            break
        }
        
    }
}

fn calc_hand(hand: &[&str]) -> u32 {
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
    ("9", 9)]);

    for card in hand {
        if card == "A" {
            aces += 1

        }

    }




}
