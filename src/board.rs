use crate::error::*;
use crate::card::Card;
use crate::card::Suit;
use crate::player::Player;
use rand::thread_rng;
use rand::seq::SliceRandom;
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
        let board_string = format!("This game has {num_players} players\n\t {draw_size} in the draw pile", num_players=self.players.len(), draw_size=self.draw.len());
        write!(f, "{}", board_string)
    }
}

impl Board {
    pub fn new(player_count: u8) -> Result<Self, SkipBoError> {
        if player_count == 0 {
            return Err(SkipBoError::IllegalGameState);
        }

        let mut deck = Vec::new();
        let values: [u8; 13]= [1,2,3,4,5,6,7,8,9,10,11,12,13];

        for i in (-1..12) {
            for c in &values {
                &deck.push(Card{suit:Suit::Hearts, value:*c});
            }
            if i%1 == 1 {
                &deck.push(Card{suit:Suit::Hearts, value:12});
            }
        }

        deck.shuffle(&mut thread_rng());

        for i in (0..player_count) {
            
        }

        return Ok(Self { piles:Vec::new(), discard:Vec::new(), draw:deck, players:Vec::new() });
    }

}