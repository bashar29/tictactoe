mod board;
mod player;
//use anyhow::{Context,Result};
use log::{info, error};

//pub type Error = anyhow::Error;
//pub type Result<T> = anyhow::Result<T>;

fn main() {
    env_logger::init();
    info!("Lauching the game ...");
    let board: board::Board = board::generate_new_board();
    let output = board::render_board(board).unwrap();
    println!("{}",output);

    match player::get_move() {
        Ok(t) => info!("your move : {:?}",t),
        Err(e) => error!("{}",e),
    }
}
