use super::bytes::Puzzlehash;
use std::fmt;

#[derive(Debug)]
pub struct PuzzlehashNotFound(pub Puzzlehash);

impl fmt::Display for PuzzlehashNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Keychain mismatch for puzzlehash: {:?}", self.0)
    }
}

impl std::error::Error for PuzzlehashNotFound {}
