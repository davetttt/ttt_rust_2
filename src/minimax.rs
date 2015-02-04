use super::token::Token;
use super::rules;
use super::board;

fn get_score(board: &Vec<Token>) -> isize {
    let absolute_value_score = board.len() as isize + 1;
    match rules::get_winner(board) {
        Some(Token::X) => absolute_value_score,
        Some(Token::O) => -absolute_value_score,
        _              => 0,
    }
}

fn do_weighting(score: isize) -> isize {
    if score > 0 {
        score - 1
    } else if score < 0 {
        score + 1
    } else {
        score
    }
}

pub fn minimax(board: &Vec<Token>) -> (Option<usize>, isize) {
    if rules::game_is_over(board) {
        (None, get_score(board))
    } else {
        let token = rules::get_turn_token(board);
        let mut space_score_tuples = vec![];
        for space in board::empty_spaces(board).iter() {
            let possible_board = board::set_space(board, *space, token);
            let score = do_weighting(minimax(&possible_board).1);
            space_score_tuples.push((Some(*space), score));
        }
        let best_tuple = match token {
            Token::X => *space_score_tuples.iter().
                            max_by(|tuple| tuple.1).unwrap(),
            _        => *space_score_tuples.iter().
                            min_by(|tuple| tuple.1).unwrap(),
        };
        best_tuple
    }
}

