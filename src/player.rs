use std::fmt;
use crate::card::Card;
use crate::error::SkipBoError;

#[derive(Debug)]
pub struct Player {
    hand: Vec<Card>,
    discard_piles: Vec<Vec<Card>>,
    game_pile: Vec<Card>
}

impl Player {
    fn new(game_pile: Vec<Card>) -> Player {
        game_pile = game_pile;
        discard_piles = Vec::new();
        hand = Vec::new();
        return Self{ hand, discard_piles, game_pile};
    }

    fn deal(&mut self, card: Card) -> Result<(), SkipBoError> {
        if self.hand.len() > 4 {
            return Err(SkipBoError::IllegalGameState)
        }
        self.hand.push(card);
        return Ok(());
    }

}