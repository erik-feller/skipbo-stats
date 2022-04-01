mod card;
mod board;
mod player;
mod error;

use rand::thread_rng;
use rand::seq::SliceRandom;

use std::iter;
use crate::card::Card;
use crate::board::Board;

fn main() {
    let board_test = Board::new(4);
    let test_str = String::from(format!("this is a test of the {var} substitution", var = "variable"));
    match board_test {
        Ok(b) => println!("{}", b),
        Err(e) => println!("{}", e),
    }
}
