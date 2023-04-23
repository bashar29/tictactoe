use std::io;

use anyhow::{Result, bail};
use log::debug;
use crate::{player::{Player, self}, board};

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
            }
            else {
                println!("Illegal move - try again");
            }
        }
        if board::is_move_win(&board) {
            return Some(active_player);
        }
        else {
            if active_player == Player::PlayerX {
                active_player = Player::PlayerO
            }
            else{
                active_player = Player::PlayerX
            }
        }
    }
    None
}