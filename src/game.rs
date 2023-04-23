use std::io;

use crate::{
    board,
    player::{self, Player},
};
use anyhow::{bail, Result};
use log::debug;

pub fn play_game() -> Option<Player> {
    debug!("Launching a new game");

    let mut board: board::Board = board::generate_new_board();
    let output = board::render_board(board).unwrap();
    println!("{}", output);

    let mut full_cases: u8 = 0;
    let mut active_player = Player::PlayerX;

    while full_cases < 9 {
        loop {
            let mv = player::get_move(&mut io::stdin().lock()).unwrap();
            if board::is_valid_move(&board, mv) {
                board = board::make_move(&board, mv, &active_player).unwrap();
                let output = board::render_board(board).unwrap();
                println!("{}", output);
                full_cases += 1;
                break;
            } else {
                println!("Illegal move - try again");
            }
        }
        match board::is_move_win(&board) {
            Some(p) => return Some(p),
            None => active_player = switch_player(&active_player),
        }
    }
    None
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
    pub fn test_switch_player() {
        init();
        let active_player = Player::PlayerO;
        let next_player = switch_player(&active_player);
        assert_ne!(active_player,next_player);
    }
}