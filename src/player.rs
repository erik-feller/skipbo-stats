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
        let discard_piles = vec![];
        let hand = vec![];
        return Self{ hand, discard_piles, game_pile};
    }

    fn deal(&mut self, card: Card) -> Result<(), SkipBoError> {
        if self.hand.len() > 4 {
            return Err(SkipBoError::IllegalGameState)
        }
        self.hand.push(card);
        return Ok(());
    }

    fn discard(&mut self, pile: usize, card: Card) -> Result<(), SkipBoError> {
        if pile > 3 {
            return Err(SkipBoError::OverfullHand)
        }
        while self.discard_piles.len() < pile+1 {
            self.discard_piles.push(vec![])
        }

        self.discard_piles[pile].push(card);
        return Ok(())

    }

}