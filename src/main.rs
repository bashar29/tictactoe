mod board;
mod player;
//use anyhow::{Context,Result};
use log::info;
use std::io;

use crate::board::render_board;

//pub type Error = anyhow::Error;
//pub type Result<T> = anyhow::Result<T>;

fn main() {
    env_logger::init();
    info!("Lauching the game ...");
    let mut board: board::Board = board::generate_new_board();
    let output = board::render_board(board).unwrap();
    println!("{}", output);
    loop {
        let mv = player::get_move(&mut io::stdin().lock()).unwrap();
        if board::is_valid_move(&board, mv) {
            board = board::make_move(&board, mv, &player::Player::PlayerO).unwrap();
            let output = render_board(board).unwrap();
            println!("{}", output);
            break;
        }
        else {
            println!("Illegal move - try again");
        }
    }
}