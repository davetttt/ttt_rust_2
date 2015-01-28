use ::board::*;
use ::token::Token;
 
fn set_spaces(board: Vec<Token>, spaces: Vec<usize>, token: Token) -> Vec<Token> {
    let mut resulting_board = board;
    for space in spaces.iter() {
        resulting_board[*space] = token;
    }
    resulting_board
}

fn generate_test_board() -> Vec<Token> {
    let empty_board = generate_empty_board(3);
    let temp_board = set_spaces(empty_board, vec![0, 2, 7], Token::X);
    set_spaces(temp_board, vec![4, 5], Token::O)
}

#[test]
fn test_generate_empty_board() {
    // with width 3, returns vector of 9 empty tokens
    let board = generate_empty_board(3);
    let expected = vec![Token::Empty, Token::Empty, Token::Empty,
                        Token::Empty, Token::Empty, Token::Empty,
                        Token::Empty, Token::Empty, Token::Empty];

    assert_eq!(board, expected);
}

#[test]
fn test_set_space() {
    // returns copy of the board with space set with token
    let board = set_space(generate_empty_board(3), 4, Token::X);
    let expected = vec![Token::Empty, Token::Empty, Token::Empty,
                        Token::Empty, Token::X, Token::Empty,
                        Token::Empty, Token::Empty, Token::Empty];
    assert_eq!(board, expected);
}

#[test]
fn test_get_rows() {
    // with 9 space board, returns vector of 3 vectors, one for each row
    let board = generate_test_board();
    let expected = vec![vec![Token::X,     Token::Empty, Token::X],
                        vec![Token::Empty, Token::O,     Token::O],
                        vec![Token::Empty, Token::X,     Token::Empty]];

    assert_eq!(get_rows(board), expected);
}

#[test]
fn test_get_columns_one() {
    // with 9 space board, returns vector of 3 vectors, one for each column
    let board = generate_test_board();
    let expected = vec![vec![Token::X,     Token::Empty, Token::Empty],
                        vec![Token::Empty, Token::O,     Token::X],
                        vec![Token::X,     Token::O,     Token::Empty]];

    assert_eq!(get_columns(board), expected);
}

#[test]
fn test_get_columns_two() {
    // works with a second 9 space board
    let empty_board = generate_empty_board(3);
    let temp_board = set_spaces(empty_board, vec![1, 4, 6], Token::X);
    let board = set_spaces(temp_board, vec![0, 8], Token::O);

    let expected = vec![vec![Token::O,     Token::Empty, Token::X],
                        vec![Token::X,     Token::X,     Token::Empty],
                        vec![Token::Empty, Token::Empty, Token::O]];
    assert_eq!(get_columns(board), expected);
}

