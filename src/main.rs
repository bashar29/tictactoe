mod board;
//use anyhow::{Context,Result};
use log::info;

//pub type Error = anyhow::Error;
//pub type Result<T> = anyhow::Result<T>;

fn main() {
    env_logger::init();
    info!("Lauching the game ...");
    let board: board::Board = board::generate_new_board();
    board::render_board(board);
}
