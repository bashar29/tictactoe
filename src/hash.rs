use crate::board::Board;
use crate::player::Player;
use std::hash::{Hash, Hasher};

#[derive(Debug, Hash)]
struct Position<'a> {
    board: &'a Board,
    player: &'a Player,
}

impl Position<'_> {
    fn bytes(&self) -> Vec<u8> {
        let mut b: Vec<u8> = Vec::new();
        for r in self.board.iter() {
            for c in r.iter() {
                if let Some(p) = c {
                    b.push(*p as u8);
                } else {
                    b.push(0);
                }
            }
        }
        b
    }
}

pub fn compute_cache(board: &Board, player: &Player) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    let s = Position { board, player };
    hasher.write(&s.bytes());
    hasher.finish()
}
