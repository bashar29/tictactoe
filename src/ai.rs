use crate::board::{Board, self};
use crate::player::Player;
use anyhow::Result;
use log::debug;
use rand::Rng;

pub fn random_ai(board: &Board, player: &Player) -> Result<Board> {
    debug!("Random AI move generation");
    let mut legal_moves = Vec::new();
    for (y, line) in board.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if c.is_none() {
                legal_moves.push((y, x));
            }
        }
    }
    let mut rng = rand::thread_rng();
    let chosen_move = legal_moves[rng.gen_range(0..legal_moves.len())];
    debug!("{:?}",(chosen_move.1,chosen_move.0));
    let new_board = board::make_move(board, chosen_move, player).unwrap();

    Ok(new_board)
}

#[cfg(test)]
mod tests {

    use super::*;
    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    pub fn test_random_ai() {
        init();
        let board: [[Option<char>; 3]; 3] = [
            [Some('O'), Some('X'), None],
            [None, Some('O'), None],
            [None, Some('X'), Some('X')],
        ];
        for _ in 0..33 {
            let new_board = random_ai(&board, &Player::PlayerO).unwrap();
            assert_ne!(new_board, board);
        }
    }
}