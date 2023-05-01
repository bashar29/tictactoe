pub type Result<T> = anyhow::Result<T>;
use crate::board::Board;
use crate::{ai, player};
use crate::{board, player::Player};
use log::debug;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Engine {
    Human,
    RandomMove,
    WinningMove,
    WinningAndNotLosingMove,
}

pub fn play_game(player_x_engine: Engine, player_o_engine: Engine) -> Option<Player> {
    debug!("Launching a new game");

    let mut board: Board = board::generate_new_board();
    let mut output = board::render_board(&board).unwrap();
    println!("{}", output);

    let mut full_cases: u8 = 0;
    let mut active_player = Player::PlayerX;
    let mut active_engine = player_x_engine;

    while full_cases < 9 {
        board = play_move(&board, &active_player, &active_engine).unwrap();
        full_cases += 1;
        output = board::render_board(&board).unwrap();
        println!("{}", output);

        match board::is_move_win(&board) {
            Some(p) => return Some(p),
            None => {
                active_player = switch_player(&active_player);
                if active_engine == player_x_engine {
                    active_engine = player_o_engine;
                    debug!("active engine : {:?}", active_engine)
                } else {
                    active_engine = player_x_engine;
                    debug!("active engine : {:?}", active_engine)
                }
            }
        }
    }
    None
}

fn play_move(board: &Board, active_player: &Player, engine: &Engine) -> Result<Board> {
    match engine {
        Engine::Human => player::human_get_move(board, active_player),
        Engine::RandomMove => ai::random_ai(board, active_player),
        Engine::WinningMove => ai::finds_winning_moves_ai(board, active_player),
        Engine::WinningAndNotLosingMove => {
            ai::finds_winning_and_not_losing_moves_ai(board, active_player)
        }
    }
}

fn switch_player(active_player: &Player) -> Player {
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
        assert_ne!(active_player, next_player);
    }
}
