use log::debug;
use anyhow::{Result, bail};
use std::io;


pub fn get_move() -> Result<(u8,u8)> {
    debug!("Get player's move from keyboard");
    
    println!("\nPlease input your move. Format : x_coord (from 0 to 2) , y_coord (from 0 to 2)");
    println!("Example : > 1,2");
    println!("Coordinates : ");
    let board_example = "  0 1 2\n".to_owned() + " -------\n" + "0      \n" + "1      \n" + "2      \n";
    println!("{}", board_example);

    let mut player_input = String::new();
    io::stdin().read_line(&mut player_input)?;
    let player_move = get_input_from_keyboard(&player_input)?;
    //TODO : traiter ici l'erreur ou "plus haut" ?
    Ok(player_move)
}

fn get_input_from_keyboard(player_input: &str) -> Result<(u8,u8)> {
    debug!("Transform keyboard input in tuple");
    let mut player_move:(u8,u8) = (u8::MAX,u8::MAX);
    for s in player_input.trim().split(",") {
        if player_move.0 == u8::MAX {
            player_move.0 = s.trim().parse()?;
            // TODO : personnaliser l'erreur
        }
        else {
            player_move.1 = s.trim().parse()?;
            // TODO : personnaliser l'erreur 
        }
    }
    debug!("Move : {:?}",player_move);
    if player_move.0 > 2 || player_move.1 > 2 {
        bail!("Coordinates not included in [0..2]");
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
        assert_eq!((1,2),get_input_from_keyboard(input).unwrap());
    }

    #[test]
    #[should_panic(expected = "invalid digit found in string")]
    pub fn test_get_negative_input_from_keyboard() {
        init();
        let input = "-1,2";
        assert_eq!((1,3),get_input_from_keyboard(input).unwrap());
    }

    #[test]
    #[should_panic(expected = "Coordinates not included in [0..2]")]
    pub fn test_get_bad_input_from_keyboard() {
        init();
        let input = "1,3";
        assert_eq!((1,3),get_input_from_keyboard(input).unwrap());
        assert_eq!((1,3),get_input_from_keyboard(input).unwrap());
    }


}