use crate::board::Board;
use crate::player::Player;
use anyhow::Result;
use rand::Rng;

pub fn random_ai(board: &Board, player: &Player) -> Result<(usize, usize)> {
    let mut legal_moves = Vec::new();
    for (y, line) in board.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if c.is_none() {
                legal_moves.push((x, y));
            }
        }
    }
    let mut rng = rand::thread_rng();
    let chosen_move = legal_moves[rng.gen_range(0..legal_moves.len())];
    Ok(chosen_move)
}

#[cfg(test)]
mod tests {
    use log::debug;

    use super::*;
    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    pub fn test_random_ai() {
        init();
        let board: [[Option<char>; 3]; 3] = [
            [Some('O'), Some('X'), Some('O')],
            [None, Some('O'), None],
            [None, Some('X'), Some('X')],
        ];
        for _ in 0..33 {
            let mv = random_ai(&board, &Player::PlayerO).unwrap();
            debug!("{:?}",mv);
            assert_ne!(mv, (0,0));
            assert_ne!(mv, (1,0));
            assert_ne!(mv, (2,0));
            assert_ne!(mv, (1,1));
            assert_ne!(mv, (1,2));
            assert_ne!(mv, (2,2));
        }
    }

}
