// Module import
use rand::prelude::*;
use prompted::input;

fn main() {

    let mut rng = rand::rng();

    let mut deck = ["A","K", "Q", "J", "10", "9", "8", "7", "6", "5", "4", "3", "2","A","K", "Q", "J", "10", "9", "8", "7", "6", "5", "4", "3","2","A","K", "Q", "J", "10", "9", "8", "7", "6", "5", "4", "3", "2","A","K", "Q", "J", "10", "9", "8", "7", "6", "5", "4", "3", "2"];

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
