use super::board;
use super::token::Token;

fn winner_for_line(line: &Vec<Token>) -> Option<Token> {
    if line[0] != Token::Empty && line.iter().all(|token| *token == line[0]) {
        Some(line[0])
    } else {
        None
    }
}

fn get_winner(board: &Vec<Token>) -> Option<Token> {
    let mut lines = board::get_rows(board);
    lines.push_all(board::get_columns(board).as_slice());
    lines.push_all(board::get_diagonals(board).as_slice());

    let mut winner = None;
    for line in lines.iter() {
        winner = winner_for_line(line);
        if winner.is_some() {
            break;
        }
    }
    winner
}

#[allow(unused_variables)] // x is not used
fn has_winner(board: &Vec<Token>) -> bool {
    match get_winner(board) {
        Some(x) => true,
        None    => false
    }
}

pub fn game_is_over(board: &Vec<Token>) -> bool {
    board::board_is_full(board) || has_winner(board)
}
