use anyhow::{Result, bail};
use log::debug;

use crate::player::Player;

pub type Board = [[Option<char>; 3]; 3];

pub fn generate_new_board() -> Board {
    debug!("Generate a new and clean board");
    [[None, None, None], [None, None, None], [None, None, None]]
}

fn duplicate_board(board: &Board) -> Board {
    let mut new_board = generate_new_board();
    for (i, line) in board.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            new_board[i][j] = *c;
        }
    }
    new_board
}

pub fn is_valid_move(board: &Board,new_move: (usize,usize)) -> bool {
    board[new_move.0][new_move.1].is_none()
}

pub fn is_move_win(board: &Board) -> bool {
    // TODO
    false
}

pub fn render_board(board: Board) -> Result<String> {
    debug!("Render a board : {:?}", board);
    let mut output = "  0 1 2\n".to_owned();
    output += " -------\n";

    for (i, line) in board.iter().enumerate() {
        let mut line_to_print: String = i.to_string();
        line_to_print.push_str(" ");
        line_to_print.push(line[0].unwrap_or_else(|| ' '));
        line_to_print.push_str(" ");
        line_to_print.push(line[1].unwrap_or_else(|| ' '));
        line_to_print.push_str(" ");
        line_to_print.push(line[2].unwrap_or_else(|| ' '));
        line_to_print.push('\n');
        output.push_str(&line_to_print);
    }
    Ok(output)
}

pub fn make_move(board: &Board, new_move: (usize, usize), player: &Player) -> Result<Board> {
    if !is_valid_move(&board, new_move) {
        bail!("Invalid move !");
    }
    let mut new_board = duplicate_board(board);
    match player {
        Player::PlayerO => new_board[new_move.0][new_move.1] = Some('O'),
        Player::PlayerX => new_board[new_move.0][new_move.1] = Some('X'),
    }
    Ok(new_board)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    pub fn test_generate_new_board() {
        init();
        let expected: Board = [[None, None, None], [None, None, None], [None, None, None]];
        let actual: Board = generate_new_board();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_render_board() {
        init();
        let board: Board = [
            [Some('A'), None, Some('C')],
            [None, Some('E'), Some('F')],
            [Some('G'), Some('H'), Some('I')],
        ];

        let expected_output =
            "  0 1 2\n".to_owned() + " -------\n" + "0 A   C\n" + "1   E F\n" + "2 G H I\n";
        let output = render_board(board).unwrap();
        assert_eq!(expected_output, output);
    }

    #[test]
    pub fn test_duplicate_board() {
        init();
        let board: [[Option<char>; 3]; 3] = [
            [Some('O'), Some('X'), Some('O')],
            [None, Some('O'), Some('X')],
            [None, Some('X'), Some('X')],
        ];
        let new_board = duplicate_board(&board);
        assert_eq!(board, new_board);
    }

    #[test]
    pub fn test_make_move() {
        init();
        let mut board: [[Option<char>; 3]; 3] = [
            [Some('O'), Some('X'), Some('O')],
            [None, Some('O'), Some('X')],
            [None, Some('X'), Some('X')],
        ];
        
        let new_move: (usize, usize) = (1, 0);
        let p1 = Player::PlayerO;
        let new_board = make_move(&board, new_move, &p1).unwrap();
        board[1][0] = Some('O');
        assert_eq!(new_board, board);
        
        let new_new_move: (usize, usize) = (2, 0);
        let p2 = Player::PlayerX;
        let new_new_board = make_move(&new_board, new_new_move, &p2).unwrap();
        board[2][0] = Some('X');
        assert_eq!(new_new_board, board);
        
        let last_move = (2,0);
        let error: anyhow::Error = make_move(&new_new_board, last_move, &p1).unwrap_err();
        let expected_error: anyhow::Error = anyhow::anyhow!("Invalid move !");
        assert_eq!(error.to_string(), expected_error.to_string());
    }

    #[test]
    pub fn test_is_valid_move() {
        init();
        let board: [[Option<char>; 3]; 3] = [
            [Some('O'), Some('X'), Some('O')],
            [None, None, Some('X')],
            [None, Some('X'), None],
        ];
        let mv: (usize,usize) = (1,1);
        assert_eq!(true,is_valid_move(&board, mv));
        let mv: (usize,usize) = (0,0);
        assert_eq!(false,is_valid_move(&board, mv));
    }
}
