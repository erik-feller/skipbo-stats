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
    fn new(&mut self, game_pile: Vec<Card>) -> &mut Player {
        self.game_pile = game_pile;
        self.discard_piles = Vec::new();
        self.hand = Vec::new();
        return self;
    }

    fn deal(&mut self, card: Card) -> Result<(), SkipBoError> {
        if self.hand.len() > 4 {
            return Err(SkipBoError::IllegalGameState)
        }
        self.hand.push(card);
        return Ok(());
    }

}