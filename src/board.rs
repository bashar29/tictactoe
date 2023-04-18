//use anyhow::{Context,Result};
use log::debug;

pub type Board = [[Option<char>; 3]; 3];

pub fn generate_new_board() -> Board {
    debug!("Generate a new and clean board");
    [[None, None, None], [None, None, None], [None, None, None]]
}

pub fn render_board(board: Board) {
    debug!("Render a board : {:?}", board);
    println!("  0 1 2 ");
    println!(" -------");
    for (i, line) in board.iter().enumerate() {
        let mut line_to_print: String = i.to_string();
        line_to_print.push_str(" ");
        line_to_print.push(line[0].unwrap_or_else(|| ' '));
        line_to_print.push_str(" ");
        line_to_print.push(line[1].unwrap_or_else(|| ' '));
        line_to_print.push_str(" ");
        line_to_print.push(line[2].unwrap_or_else(|| ' '));
        println!("{}", line_to_print);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_generate_new_board() {
        env_logger::init();

        let expected: Board = [[None, None, None], [None, None, None], [None, None, None]];
        let actual: Board = generate_new_board();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_render_board() {

        let board: Board = [
            [Some('A'), None, Some('C')],
            [None, Some('E'), Some('F')],
            [Some('G'), Some('H'), Some('I')],
        ];
        render_board(board);
    }
}
