mod card;
mod board;
mod player;
mod error;

use rand::thread_rng;
use rand::seq::SliceRandom;

use std::iter;
use crate::card::Card;
use crate::card::Suit;
use crate::board::Board;

fn main() {
    let mut deck = Vec::new();
    let values: [u8; 13]= [1,2,3,4,5,6,7,8,9,10,11,12,13];

    for i in (0..12) {
        for c in &values {
            &deck.push(Card{suit:Suit::Hearts, value:*c});
        }
        if i%2 == 1 {
            &deck.push(Card{suit:Suit::Hearts, value:13});
        }
    }

    deck.shuffle(&mut thread_rng());

    let test_str = String::from(format!("this is a test of the {var} substitution", var = "variable"));
    println!("{}", test_str);
}
