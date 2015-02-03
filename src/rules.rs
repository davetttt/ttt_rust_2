use super::board;
use super::token::Token;

fn winner_for_line(line: &Vec<Token>) -> Option<Token> {
    if line[0] != Token::Empty && line.iter().all(|token| *token == line[0]) {
        Some(line[0])
    } else {
        None
    }
}

pub fn get_winner(board: &Vec<Token>) -> Option<Token> {
    let mut lines = board::get_rows(board);
    lines.push_all(board::get_columns(board).as_slice());
    lines.push_all(board::get_diagonals(board).as_slice());
    match lines.iter().
              map(|line| winner_for_line(line)).
              find(|winner| winner.is_some()) {
        Some(x) => x,
        None    => None,
    }
}

#[allow(unused_variables)]
fn has_winner(board: &Vec<Token>) -> bool {
    get_winner(board).is_some()
}

pub fn game_is_over(board: &Vec<Token>) -> bool {
    board::is_full(board) || has_winner(board)
}

pub fn turn(board: &Vec<Token>) -> Token {
    if board::count_tokens(board, Token::X) > board::count_tokens(board, Token::O) {
        Token::O
    } else {
        Token::X
    }
}
