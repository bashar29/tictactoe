pub type Result<T> = anyhow::Result<T>;
use std::collections::HashMap;

use crate::board::Board;
use crate::{ai, player};
use crate::{board, player::Player};
use log::info;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Engine {
    Human,
    RandomMove,
    WinningMove,
    WinningAndNotLosingMove,
    MinMax,
}

pub fn play_game(player_x_engine: Engine, player_o_engine: Engine) -> Option<Player> {
    info!("Launching a new game");

    let mut board: Board = board::generate_new_board();
    let mut output = board::render_board(&board).unwrap();
    println!("{}", output);

    let mut full_cases: u8 = 0;
    let mut active_player = Player::PlayerX;
    let mut active_engine = player_x_engine;

    let mut cache: HashMap<u64, i8> = HashMap::new();

    while full_cases < 9 {
        board = play_move(&board, &active_player, &active_engine, &mut cache).unwrap();
        full_cases += 1;
        output = board::render_board(&board).unwrap();
        println!("{}", output);

        match board::is_move_win(&board) {
            Some(p) => return Some(p),
            None => {
                active_player = switch_player(&active_player);
                if active_engine == player_x_engine {
                    active_engine = player_o_engine;
                } else {
                    active_engine = player_x_engine;
                }
            }
        }
    }
    None
}

fn play_move(
    board: &Board,
    active_player: &Player,
    engine: &Engine,
    cache: &mut HashMap<u64, i8>,
) -> Result<Board> {
    match engine {
        Engine::Human => player::human_get_move(board, active_player),
        Engine::RandomMove => ai::random_ai(board, active_player),
        Engine::WinningMove => ai::finds_winning_moves_ai(board, active_player),
        Engine::WinningAndNotLosingMove => {
            ai::finds_winning_and_not_losing_moves_ai(board, active_player)
        }
        Engine::MinMax => ai::minimax_algo_ai(board, active_player, cache),
    }
}

pub fn switch_player(active_player: &Player) -> Player {
    match active_player {
        Player::PlayerX => Player::PlayerO,
        Player::PlayerO => Player::PlayerX,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_switch_player() {
        init();
        let active_player = Player::PlayerO;
        let next_player = switch_player(&active_player);
        assert_ne!(active_player, next_player);
    }

    #[test]
    fn test_play_move() {
        init();
        let active_player = Player::PlayerX;
        let board: Board = [
            [Some('X'), None, Some('O')],
            [None, Some('O'), Some('X')],
            [Some('X'), None, Some('0')],
        ];
        let engine = Engine::RandomMove;
        let mut cache: HashMap<u64, i8> = HashMap::new();
        let new_board = play_move(&board, &active_player, &engine, &mut cache).unwrap();
        assert!(new_board[0][1] == Some('X') || new_board[1][0] == Some('X') || new_board[2][1] == Some('X'));
        cache.clear();
        
        let engine = Engine::MinMax;
        let active_player = Player::PlayerO;
        let new_new_board = play_move(&new_board, &active_player, &engine, &mut cache).unwrap();
        assert!(new_new_board[0][1] == Some('O') || new_new_board[1][0] == Some('O') || new_new_board[2][1] == Some('O'));

    }
}
