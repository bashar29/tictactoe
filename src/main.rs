mod ai;
mod board;
mod game;
mod hash;
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
    /// Number of iteration
    #[arg(short, long)]
    i: usize,
}

fn main() {
    env_logger::init();
    info!("Launching tictactoe");
    let args = Args::parse();
    let engine_x: Engine = get_engine_from_arg(&args.x);
    let engine_o: Engine = get_engine_from_arg(&args.o);
    player::print_player_input_rule();

    let iteration_number = args.i;
    let mut results: (usize, usize, usize) = (0, 0, 0);

    for _i in 0..iteration_number {
        match game::play_game(engine_x, engine_o) {
            Some(p) => {
                println!("Well done {:?} !!!", p);
                match p {
                    player::Player::PlayerX => results.0 += 1,
                    player::Player::PlayerO => results.1 += 1,
                }
            }
            None => {
                println!("This is a draw !");
                results.2 += 1
            }
        }
    }
    println!(
        "X win {} ; O win {} ; draw {}.",
        results.0, results.1, results.2
    );
}

fn get_engine_from_arg(arg: &str) -> Engine {
    match arg {
        "Human" => Engine::Human,
        "RandomMove" => Engine::RandomMove,
        "WinningMove" => Engine::WinningMove,
        "WinningAndNotLosingMove" => Engine::WinningAndNotLosingMove,
        "MinMax" => Engine::MinMax,
        _ => {
            println!("Unknown engine passed by args ; RandomMove selected.");
            Engine::RandomMove
        }
    }
}
