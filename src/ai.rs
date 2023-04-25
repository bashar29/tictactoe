use crate::board::{Board, self};
use crate::player::Player;
use anyhow::{Result, anyhow};
use log::debug;
use rand::Rng;

pub fn random_ai(board: &Board, player: &Player) -> Result<Board> {
    debug!("Random AI move generation");
    let legal_moves = find_all_legal_moves(board);
    let new_board = match select_one_random_move(&legal_moves, board, player) {
        Some(b) => b,
        None => return Err(anyhow!("no legal move available")),
    };
    Ok(new_board)
}

pub fn finds_winning_moves_ai(board: &Board, player: &Player) -> Result<Board> {
    debug!("Search an immediate winning move");
    let legal_moves = find_all_legal_moves(board);
    for m in &legal_moves {
        let new_board = board::make_move(board, *m, player).unwrap();
        if board::is_move_win(&new_board).is_some() {
            return Ok(new_board)
        }
    }
    let new_board = match select_one_random_move(&legal_moves, board, player) {
        Some(b) => b,
        None => return Err(anyhow!("no legal move available")),
    };
    Ok(new_board)
}

// pub fn finds_winning_and_losing_moves_ai(board: &Board, player: &Player) -> Result<Board> {

// }

fn select_one_random_move(legal_moves: &Vec<(usize,usize)>, board: &Board, player: &Player) -> Option<Board> {
    let mut rng = rand::thread_rng();
    let chosen_move = legal_moves[rng.gen_range(0..legal_moves.len())];
    let new_board = board::make_move(board, chosen_move, player).unwrap();
    Some(new_board)
}

fn find_all_legal_moves(board: &Board) -> Vec<(usize,usize)> {
    let mut legal_moves = Vec::new();
    for (y, line) in board.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if c.is_none() {
                legal_moves.push((y, x));
            }
        }
    }
    legal_moves
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