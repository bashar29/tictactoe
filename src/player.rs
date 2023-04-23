use anyhow::{bail, Result};
use log::debug;
use std::io::BufRead;

pub enum Player {
    PlayerX,
    PlayerO,
}

pub fn print_player_input_rule() {
    debug!("Print player input rules");
    
    println!("\nPlease input your move. Format : x_coord (from 0 to 2) , y_coord (from 0 to 2)");
    println!("Example : > 1,2");
    println!("Coordinates : ");
    let board_example =
        "  0 1 2\n".to_owned() + " -------\n" + "0      \n" + "1      \n" + "2      \n";
    println!("{}", board_example);
}

pub fn get_move(input: &mut impl BufRead) -> Result<(usize, usize)> {
    debug!("Get player's move from keyboard");

    loop {
        let player_input = input.lines().next().unwrap()?;
        match get_input_from_keyboard(&player_input) {
            Ok(player_move) => return Ok(player_move),
            Err(e) => println!("Error : {} \nTry again", e),
        }
    }
}

fn get_input_from_keyboard(player_input: &str) -> Result<(usize, usize)> {
    debug!("Transform keyboard input in tuple");
    let mut player_move: (usize, usize) = (usize::MAX, usize::MAX);
    for s in player_input.trim().split(",") {
        if player_move.0 == usize::MAX {
            match s.trim().parse() {
                Ok(v) => player_move.0 = v,
                Err(e) => bail!("Coordinates not included in [0..2],[0..2] - {}", e),
            }
        } else {
            match s.trim().parse() {
                Ok(v) => player_move.1 = v,
                Err(e) => bail!("Coordinates not included in [0..2],[0..2] - {}", e),
            }
        }
    }
    debug!("Move : {:?}", player_move);
    if player_move.0 > 2 || player_move.1 > 2 {
        bail!("Coordinates not included in [0..2],[0..2]");
    }
    Ok(player_move)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    pub fn test_get_input_from_keyboard() {
        init();
        let input = "1,2";
        assert_eq!((1, 2), get_input_from_keyboard(input).unwrap());
        let input = "1 , 2";
        assert_eq!((1, 2), get_input_from_keyboard(input).unwrap());
        let input = "1, 2 ";
        assert_eq!((1, 2), get_input_from_keyboard(input).unwrap());
        let input = " 1 , 2";
        assert_eq!((1, 2), get_input_from_keyboard(input).unwrap());
    }

    #[test]
    #[should_panic(expected = "Coordinates not included in [0..2]")]
    pub fn test_get_bad_input_from_keyboard() {
        init();
        let input = "1,3";
        get_input_from_keyboard(input).unwrap();
        let input = "-1,2";
        get_input_from_keyboard(input).unwrap();
    }

    #[test]
    pub fn test_get_move() {
        let mut input = "1,2\n".as_bytes();
        assert_eq!((1, 2), get_move(&mut input).unwrap());
        let mut input = "2 ,2 \n".as_bytes();
        assert_eq!((2, 2), get_move(&mut input).unwrap());
        let mut input = "2,0\n".as_bytes();
        assert_eq!((2, 0), get_move(&mut input).unwrap());
        let mut input = "2,0\n 3,5".as_bytes();
        assert_eq!((2, 0), get_move(&mut input).unwrap());
        let mut input = "2,0\n 1,1".as_bytes();
        assert_eq!((2, 0), get_move(&mut input).unwrap());
    }
}
