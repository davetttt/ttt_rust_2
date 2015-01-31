use super::board;
use super::token::Token;

fn winner_for_line(line: &Vec<Token>) -> Token {
    if line[0] != Token::Empty && line.iter().all(|token| *token == line[0]) {
        line[0]
    } else {
        Token::Empty
    }
}

pub fn get_winner(board: &Vec<Token>) -> Token {
    let mut lines = board::get_rows(board);
    lines.push_all(board::get_columns(board).as_slice());
    lines.push_all(board::get_diagonals(board).as_slice());
    let winner = lines.iter().
                     map(|line| winner_for_line(line)).
                     find(|winner| *winner != Token::Empty);
    match winner {
        Some(token) => token,
        None        => Token::Empty,
    }
}

fn has_winner(board: &Vec<Token>) -> bool {
    match get_winner(board) {
        Token::Empty => false,
        _            => true,
    }
}

pub fn game_is_over(board: &Vec<Token>) -> bool {
    board::board_is_full(board) || has_winner(board)
}
