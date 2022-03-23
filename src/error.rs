use thiserror::Error;

#[derive(Error, Debug)]
pub enum SkipBoError {

    //Error for when the gamestate would become illegal. These shouldn't happen often at all but safety first IG
    #[error("Action would cause an illegal game state")]
    IllegalGameState
}