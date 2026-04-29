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

    println!("{:?}", deck);
    println!("{:?}", player_hand);
    println!("{:?}", dealer_hand);

    // Setup

        match calc_hand(&player_hand).1 { 
            false => println!("Player hand: {:?} -> {}", player_hand, calc_hand(&player_hand).0),
            true => println!("Player hand: {:?} -> {} or {}",player_hand, calc_hand(&player_hand).0, calc_hand(&player_hand).0 - 10)
        };

        println!("Dealer hand: [{}, ?]", dealer_hand[0]); // hide dealer's second card

        if calc_hand(&dealer_hand).0 == 21 { // check for BJ
            println!("Dealer has {:?}, Blackjack!", dealer_hand);
            if calc_hand(&player_hand).0 == 21 {
                println!("You also have blackjack with {:?}! Insane!", player_hand); // tiny tiny change of this happening
            }          
            println!("Restarting...");
            main();  

        }

        loop { //player loop
        let action = input!("Would you like to hit (H) or stand (S)?");
        // Implement actions for cards

        if action == "H" {
            player_hand.push(deck.pop().expect("empty"));
            let score = calc_hand(&player_hand); //draw and calc score.
            
            if score.0 > 21 {
                println!("Bust! Restarting game loop!");
                main();
            } 

            match score.1 { 
            false => println!("Player hand: {:?} -> {}", player_hand, calc_hand(&player_hand).0),
            true => println!("Player hand: {:?} -> {} or {}",player_hand, calc_hand(&player_hand).0, calc_hand(&player_hand).0 - 10)
            };

        } else if action == "S" {
            break;
        }
        else {
            print!("Invalid input, please enter H or S");
            break;
        }
        
        } // Close player loop
        
        loop {
        let (value, soft) = calc_hand(&dealer_hand);

        println!("Dealer hand: {:?} -> {}", dealer_hand, value);

        if value > 21 {
            println!("Dealer busts! Player wins.");
            return;
        }

        let should_hit = value < 17 || (value == 17 && soft);

        if should_hit {
            dealer_hand.push(deck.pop().expect("empty"));
        } else {
            break;
        } 
    } //end dealer loop
        
    let (player_value, _) = calc_hand(&player_hand);
    let (dealer_value, _) = calc_hand(&dealer_hand);

    println!("\n--- Result ---");
    println!("Player: {}", player_value);
    println!("Dealer: {}", dealer_value);

    match player_value.cmp(&dealer_value) {
        std::cmp::Ordering::Greater => println!("Player wins!"),
        std::cmp::Ordering::Less    => println!("Dealer wins!"),
        std::cmp::Ordering::Equal   => println!("Push!"),
    }// select winner

    println!("Restarting...");
    main();
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







