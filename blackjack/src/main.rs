// Module import
use rand::prelude::*;
use prompted::input;

fn main() {
    // Very first test
    let mut x = 5;
    x = 10;
    println!("{}",x);

    let mut rng = rand::rng();

    let deck_qu = ["A","K", "Q", "J", "10", "9", "8", "7", "6", "5", "4", "3", "2"];
    let mut deck = [deck_qu;4];
    deck.shuffle(&mut rng);

    println!("{:?}", deck);

    loop {
        let action = input!("Would you like to hit (H) or stand (S)?");

        if action == "H" {
            break
        }
    }
}
