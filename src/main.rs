mod ai;
mod board;
mod game;
mod player;
//TODO : how to add an AI like a plugin?
//use anyhow::{Context,Result};
use log::info;

//pub type Error = anyhow::Error;
//pub type Result<T> = anyhow::Result<T>;

fn main() {
    env_logger::init();
    info!("Welcome to tictactoe !!!");
    player::print_player_input_rule();

    match game::play_game() {
        Some(p) => println!("Well done {:?} !!!", p),
        None => println!("This is a draw !"),
    }
}
