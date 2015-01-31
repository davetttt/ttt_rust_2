use super::token::Token;
use super::rules;

fn get_score(board: &Vec<Token>) -> isize {
    let winner = match rules::get_winner(board) {
        Some(x) => x,
        None    => Token::Empty,
    };
    let absolute_value_score = board.len() as isize + 1;
    match winner {
        Token::X => absolute_value_score,
        Token::O => -absolute_value_score,
        _        => 0,
    }
}

pub fn minimax(board: Vec<Token>) -> (Option<usize>, isize) {
    if rules::game_is_over(&board) {
        (None, get_score(&board))
    } else {

        (None, 25)
    }
}
