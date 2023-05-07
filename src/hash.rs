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
        match self.player {
            Player::PlayerX => b.push('X' as u8),
            Player::PlayerO => b.push('O' as u8),
        };
        //log::debug!("{:?}",b);
        b
    }
}

pub fn compute_cache(board: &Board, player: &Player) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    let s = Position { board, player };
    hasher.write(&s.bytes());
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    pub fn test_compute_cache() {
        init();
        let board: Board = [
            [Some('X'), None, Some('O')],
            [None, Some('O'), Some('X')],
            [Some('X'), None, Some('O')],
        ];
        let player = Player::PlayerX;
        let mut v = Vec::new();
        const ASCII_X: u8 = 88;
        const ASCII_O: u8 = 79;
        const ASCII_NONE: u8 = 0;
        v.push(ASCII_X);v.push(ASCII_NONE);v.push(ASCII_O);
        v.push(ASCII_NONE);v.push(ASCII_O);v.push(ASCII_X);
        v.push(ASCII_X);v.push(ASCII_NONE);v.push(ASCII_O);
        v.push(ASCII_X);

        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        hasher.write(&v);
        let expected_cache = hasher.finish();

        assert_eq!(expected_cache, compute_cache(&board, &player));

    }
}
