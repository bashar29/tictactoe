mod board;
mod player;
//use anyhow::{Context,Result};
use log::{info};
use std::io;

//pub type Error = anyhow::Error;
//pub type Result<T> = anyhow::Result<T>;

fn main() {
    env_logger::init();
    info!("Lauching the game ...");
    let board: board::Board = board::generate_new_board();
    let output = board::render_board(board).unwrap();
    println!("{}",output);

    println!("{:?}",player::get_move(&mut io::stdin().lock()).unwrap());
}