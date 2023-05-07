use std::collections::HashMap;

use crate::board::{self, Board};
use crate::player::Player;
use crate::{game, hash};
use anyhow::{anyhow, Result};
use rand::Rng;

pub fn random_ai(board: &Board, player: &Player) -> Result<Board> {
    let legal_moves = find_all_legal_moves(board);
    let new_board = match select_one_random_move(&legal_moves, board, player) {
        Some(b) => b,
        None => return Err(anyhow!("no legal move available")),
    };
    Ok(new_board)
}

pub fn finds_winning_moves_ai(board: &Board, player: &Player) -> Result<Board> {
    let legal_moves = find_all_legal_moves(board);

    if let Some(b) = find_a_winning_move(&legal_moves, board, player) {
        return Ok(b);
    }

    let new_board = match select_one_random_move(&legal_moves, board, player) {
        Some(b) => b,
        None => return Err(anyhow!("no legal move available")),
    };
    Ok(new_board)
}

pub fn finds_winning_and_not_losing_moves_ai(board: &Board, player: &Player) -> Result<Board> {
    let legal_moves = find_all_legal_moves(board);

    if let Some(b) = find_a_winning_move(&legal_moves, board, player) {
        return Ok(b);
    }

    if let Some(b) = find_a_blocking_move(&legal_moves, board, player) {
        return Ok(b);
    }

    let new_board = match select_one_random_move(&legal_moves, board, player) {
        Some(b) => b,
        None => return Err(anyhow!("no legal move available")),
    };
    Ok(new_board)
}

pub fn minimax_algo_ai(
    board: &Board,
    player: &Player,
    cache: &mut HashMap<u64, i8>,
) -> Result<Board> {
    let legal_moves = find_all_legal_moves(board);
    let mut scores: Vec<(i8, Board)> = Vec::new();
    for m in legal_moves {
        let new_board = board::make_move(board, m, player).unwrap();
        let score = minimax_score(&new_board, player, cache);
        scores.push((score, new_board));
    }
    //log::debug!("{:?}", scores);

    if scores.is_empty() {
        return Err(anyhow!("no legal move available in minimax_algo_ai"));
    }

    let mut board = *board;
    match player {
        Player::PlayerX => {
            let mut score: i8 = -10;
            for s in scores {
                if s.0 > score {
                    score = s.0;
                    board = s.1;
                }
            }
            Ok(board)
        }
        Player::PlayerO => {
            let mut score: i8 = 10;
            for s in scores {
                if s.0 < score {
                    score = s.0;
                    board = s.1;
                }
            }
            Ok(board)
        }
    }
}

/// Return for a board, and a player (Player X or O), the best possible score using all legal move
fn minimax_score(board: &Board, player: &Player, cache: &mut HashMap<u64, i8>) -> i8 {
    let key = hash::compute_cache(board, player);
    if let Some(score) = cache.get(&key) {
        //log::debug!("Cache Hit");
        return *score;
    }

    if let Some(score) = minimax_score_win_or_draw(board) {
        cache.insert(key, score);
        return score;
    }

    let opponent = game::switch_player(player);
    let legal_moves = find_all_legal_moves(board);
    let mut scores: Vec<i8> = Vec::new();
    for m in legal_moves {
        let new_board = board::make_move(board, m, &opponent).unwrap();
        let score = minimax_score(&new_board, &opponent, cache);
        scores.push(score);
    }

    match opponent {
        Player::PlayerX => {
            let score = *scores.iter().max().unwrap();
            cache.insert(key, score);
            score
        }
        Player::PlayerO => {
            let score = *scores.iter().min().unwrap();
            cache.insert(key, score);
            score
        }
    }
}

/// Return score of a Board if it's a victory or a draw.
fn minimax_score_win_or_draw(board: &Board) -> Option<i8> {
    if let Some(player) = board::is_move_win(board) {
        if player == Player::PlayerX {
            return Some(10);
        } else {
            return Some(-10);
        }
    } else if board::is_board_full(board) {
        return Some(0);
    }
    None
}

fn find_a_blocking_move(
    legal_moves: &Vec<(usize, usize)>,
    board: &Board,
    active_player: &Player,
) -> Option<Board> {
    let other_player = match active_player {
        Player::PlayerX => Player::PlayerO,
        &Player::PlayerO => Player::PlayerX,
    };
    let winning_move = find_a_winning_move(legal_moves, board, &other_player);
    // if winning_move.is_none() {
    //     return None;
    // }
    winning_move?;
    let mut new_board = winning_move.unwrap();
    let diff = board::get_difference_between_board_and_next_board(board, &new_board).unwrap();
    new_board[diff.0 .1][diff.0 .0] = match active_player {
        Player::PlayerO => Some('O'),
        Player::PlayerX => Some('X'),
    };
    Some(new_board)
}

fn find_a_winning_move(
    legal_moves: &Vec<(usize, usize)>,
    board: &Board,
    player: &Player,
) -> Option<Board> {
    for m in legal_moves {
        let new_board = board::make_move(board, *m, player).unwrap();
        if board::is_move_win(&new_board).is_some() {
            return Some(new_board);
        }
    }
    None
}

fn select_one_random_move(
    legal_moves: &Vec<(usize, usize)>,
    board: &Board,
    player: &Player,
) -> Option<Board> {
    let mut rng = rand::thread_rng();
    let chosen_move = legal_moves[rng.gen_range(0..legal_moves.len())];
    let new_board = board::make_move(board, chosen_move, player).unwrap();
    Some(new_board)
}

/// return Vec of the possible (y,x) moves
fn find_all_legal_moves(board: &Board) -> Vec<(usize, usize)> {
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
    fn test_random_ai() {
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

    #[test]
    fn test_find_all_legal_moves() {
        init();
        let board: [[Option<char>; 3]; 3] = [
            [Some('O'), Some('X'), None],
            [None, Some('O'), None],
            [None, Some('X'), None],
        ];
        let legal_moves = find_all_legal_moves(&board);
        let expected_moves = vec![(0, 2), (1, 0), (1, 2), (2, 0), (2, 2)];
        assert_eq!(legal_moves, expected_moves);
    }

    #[test]
    fn test_select_one_random_move() {
        init();
        let board: [[Option<char>; 3]; 3] = [
            [Some('O'), Some('X'), None],
            [None, Some('O'), None],
            [None, Some('X'), None],
        ];
        let legal_moves = vec![(0, 2), (1, 0), (1, 2), (2, 0), (2, 2)];
        let _new_board = select_one_random_move(&legal_moves, &board, &Player::PlayerX);
        // TODO : how to assert??
    }

    #[test]
    fn test_find_winning_move() {
        init();
        let board: [[Option<char>; 3]; 3] = [
            [Some('O'), Some('X'), None],
            [None, None, Some('O')],
            [None, Some('X'), None],
        ];
        let expected_winning_board = [
            [Some('O'), Some('X'), None],
            [None, Some('X'), Some('O')],
            [None, Some('X'), None],
        ];
        let legal_moves = vec![(0, 2), (1, 0), (1, 1), (2, 0), (2, 2)];
        for _ in 1..10 {
            let winning_board = find_a_winning_move(&legal_moves, &board, &Player::PlayerX)
                .expect("unexpected ...");
            assert_eq!(expected_winning_board, winning_board);
        }
    }

    #[test]
    fn test_finds_winning_moves_ai() {
        init();
        let board: [[Option<char>; 3]; 3] = [
            [Some('O'), Some('X'), None],
            [None, None, Some('O')],
            [None, Some('X'), None],
        ];
        let expected_winning_board = [
            [Some('O'), Some('X'), None],
            [None, Some('X'), Some('O')],
            [None, Some('X'), None],
        ];
        let winning_board = finds_winning_moves_ai(&board, &Player::PlayerX).unwrap();
        assert_eq!(expected_winning_board, winning_board);
    }

    #[test]
    fn test_find_a_blocking_move() {
        init();
        let board: [[Option<char>; 3]; 3] = [
            [Some('O'), Some('X'), None],
            [None, Some('X'), Some('O')],
            [None, None, None],
        ];
        let expected_board = [
            [Some('O'), Some('X'), None],
            [None, Some('X'), Some('O')],
            [None, Some('O'), None],
        ];
        let legal_moves = vec![(0, 2), (1, 0), (2, 0), (2, 1), (2, 2)];
        let new_board = find_a_blocking_move(&legal_moves, &board, &Player::PlayerO).unwrap();
        assert_eq!(expected_board, new_board);
    }

    #[test]
    fn test_finds_winning_and_not_losing_moves_ai() {
        init();
        let board: [[Option<char>; 3]; 3] = [
            [Some('O'), Some('X'), None],
            [None, Some('X'), Some('O')],
            [None, None, None],
        ];
        let expected_board = [
            [Some('O'), Some('X'), None],
            [None, Some('X'), Some('O')],
            [None, Some('O'), None],
        ];
        //let legal_moves = vec![(0, 2), (1, 0), (2, 0), (2, 1), (2, 2)];
        let new_board = finds_winning_and_not_losing_moves_ai(&board, &Player::PlayerO).unwrap();
        assert_eq!(expected_board, new_board);

        // TODO : more variants
    }

    #[test]
    fn test_minimax_score() {
        init();
        let board = [
            [Some('O'), Some('O'), Some('X')],
            [Some('X'), Some('X'), Some('O')],
            [None, Some('O'), Some('X')],
        ];
        let active_player: Player = Player::PlayerO;
        let mut cache: HashMap<u64, i8> = HashMap::new();

        let s = minimax_score(&board, &active_player, &mut cache);
        assert_eq!(10, s);
        cache.clear();

        let full_board = [
            [Some('O'), Some('X'), Some('X')],
            [Some('X'), Some('X'), Some('O')],
            [Some('O'), Some('O'), Some('X')],
        ];
        let s = minimax_score(&full_board, &active_player, &mut cache);
        assert_eq!(0, s);
        cache.clear();

        let loosing_board = [
            [Some('O'), None, Some('X')],
            [None, Some('O'), Some('O')],
            [None, Some('X'), Some('X')],
        ];
        let s = minimax_score(&loosing_board, &active_player, &mut cache);
        assert_eq!(10, s);
        cache.clear();

        let draw_board = [
            [Some('O'), None, Some('X')],
            [Some('X'), Some('O'), Some('O')],
            [None, Some('X'), None],
        ];
        let s = minimax_score(&draw_board, &active_player, &mut cache);
        assert_eq!(0, s);
        cache.clear();
    }
}
