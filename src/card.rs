use std::fmt;

#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub value: u8
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
    
}

#[derive(Debug)]
pub enum Suit {
    Hearts,
    Spades,
    Clubs,
    Diamonds
}