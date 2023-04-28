mod ai;
mod board;
mod game;
mod player;
use game::Engine;
//TODO : how to add an AI like a plugin?
//use anyhow::{Context,Result};
use log::info;

//pub type Error = anyhow::Error;
//pub type Result<T> = anyhow::Result<T>;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Engine for player X
    #[arg(short, long)]
    x: String,
    /// Engine for player Y
    #[arg(short, long)]
    o: String,
}

fn main() {
    env_logger::init();
    info!("Welcome to tictactoe !!!");
    let args = Args::parse();
    player::print_player_input_rule();

    let engine_x: Engine;
    if args.x == "Human" {
        engine_x = Engine::Human;
    } else if args.x == "RandomMove" {
        engine_x = Engine::RandomMove;
    } else if args.x == "WinningMove" {
        engine_x = Engine::WinningMove;
    } else {
        engine_x = Engine::WinningAndNotLosingMove;
    }
    let engine_o: Engine;
    if args.o == "Human" {
        engine_o = Engine::Human;
    } else if args.o == "RandomMove" {
        engine_o = Engine::RandomMove;
    } else if args.o == "WinningMove" {
        engine_o = Engine::WinningMove;
    } else {
        engine_o = Engine::WinningAndNotLosingMove;
    }

    match game::play_game(engine_x, engine_o) {
        Some(p) => println!("Well done {:?} !!!", p),
        None => println!("This is a draw !"),
    }
}