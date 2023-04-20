use anyhow::Result;
use log::debug;

pub type Board = [[Option<char>; 3]; 3];

pub fn generate_new_board() -> Board {
    debug!("Generate a new and clean board");
    [[None, None, None], [None, None, None], [None, None, None]]
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

        let expected_output = "  0 1 2\n".to_owned() + " -------\n" + "0 A   C\n" + "1   E F\n" + "2 G H I\n";
        let output = render_board(board).unwrap();         
        assert_eq!(expected_output,output);
    }
}
