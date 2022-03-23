

use crate::card::Card;
use crate::player::Player;
use std::fmt;

#[derive(Debug)]
pub struct Board {
    piles:Vec<Vec<Card>>,
    discard:Vec<Card>,
    draw:Vec<Card>,
    players:Vec<Player>
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stub board print")
    }
}

impl Board {

}