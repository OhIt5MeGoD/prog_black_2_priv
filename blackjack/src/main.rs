// Module import
extern crate rand;
use rand::prelude::*;
use prompted::input;
use std::{collections::{HashMap, hash_map}, vec};


fn main() {

    let mut deck: Vec<&str> = Vec::from(["A", "K", "Q", "J", "10", "9", "8", "7", "6", "5", "4", "3", "2","A","K", "Q", "J", "10", "9", "8", "7", "6", "5", "4", "3","2","A","K", "Q", "J", "10", "9", "8", "7", "6", "5", "4", "3", "2","A","K", "Q", "J", "10", "9", "8", "7", "6", "5", "4", "3", "2"]);
    let mut rng = rand::rng();

    deck.shuffle(&mut rng); // Shuffle deck

    let mut dealer_hand: Vec<&str> = vec![deck.pop().expect("empty"), deck.pop().expect("empty")];
    let mut player_hand: Vec<&str> = vec![deck.pop().expect("empty"), deck.pop().expect("empty")];

    player_hand.push(deck.pop().expect("empty"));

    println!("{:?}", deck);
    println!("{:?}", player_hand);
    println!("{:?}", dealer_hand);

    loop {
        println!("The dealer is showing...");
        let action = input!("Would you like to hit (H) or stand (S)?");
        // Implement actions for cards

        if action == "H" {
            player_hand.push(deck.pop().expect("empty"));
            let score = calc_hand(&player_hand); //draw and calc score.
            
            if score.0 > 21 {
                println!("Bust! Restarting game loop!");
                main();
            } 
            break;

        } else if action == "S" {
            break
        }
        
    }
}

fn calc_hand(hand: &[&str]) -> (u32, bool) {
    let mut value: i32 = 0;
    let mut aces: i32 = 0;

    let val_map: HashMap<&str, i32> = HashMap::from([
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







