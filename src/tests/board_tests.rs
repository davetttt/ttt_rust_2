use ::board::*;
use ::token::Token;

pub fn set_spaces(board: Vec<Token>, spaces: Vec<usize>, token: Token) -> Vec<Token> {
    let mut resulting_board = board;
    for space in spaces.iter() {
        resulting_board[*space] = token;
    }
    resulting_board
}

pub fn generate_cat_board() -> Vec<Token> {
    let mut board = generate_empty_board(3);
    board = set_spaces(board, vec![0, 1, 5, 6, 8], Token::X);
    set_spaces(board, vec![2, 3, 4, 7], Token::O)
}

fn generate_test_board_one() -> Vec<Token> {
    let empty_board = generate_empty_board(3);
    let temp_board = set_spaces(empty_board, vec![0, 2, 7], Token::X);
    set_spaces(temp_board, vec![4, 5], Token::O)
}

fn generate_test_board_two() -> Vec<Token> {
    let empty_board = generate_empty_board(3);
    let temp_board = set_spaces(empty_board, vec![1, 4, 6], Token::X);
    set_spaces(temp_board, vec![0, 8], Token::O)
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
fn test_get_rows_one() {
    // with 9 space board, returns vector of 3 vectors, one for each row
    let board = generate_test_board_one();
    let expected = vec![vec![Token::X,     Token::Empty, Token::X],
                        vec![Token::Empty, Token::O,     Token::O],
                        vec![Token::Empty, Token::X,     Token::Empty]];

    assert_eq!(get_rows(&board), expected);
}

#[test]
fn test_get_rows_two() {
    // works with a second 9 space board
    let board = generate_test_board_two();
    let expected = vec![vec![Token::O,     Token::X,     Token::Empty],
                        vec![Token::Empty, Token::X,     Token::Empty],
                        vec![Token::X,     Token::Empty, Token::O]];

    assert_eq!(get_rows(&board), expected);
}

#[test]
fn test_get_columns_one() {
    // with 9 space board, returns vector of 3 vectors, one for each column
    let board = generate_test_board_one();
    let expected = vec![vec![Token::X,     Token::Empty, Token::Empty],
                        vec![Token::Empty, Token::O,     Token::X],
                        vec![Token::X,     Token::O,     Token::Empty]];

    assert_eq!(get_columns(&board), expected);
}

#[test]
fn test_get_columns_two() {
    // works with a second 9 space board
    let board = generate_test_board_two();
    let expected = vec![vec![Token::O,     Token::Empty, Token::X],
                        vec![Token::X,     Token::X,     Token::Empty],
                        vec![Token::Empty, Token::Empty, Token::O]];

    assert_eq!(get_columns(&board), expected);
}

#[test]
fn test_get_diagonals_one() {
    // with 9 space board, returns vector of 2 vectors, one for each diagonal
    let board = generate_test_board_one();
    let expected = vec![vec![Token::X, Token::O, Token::Empty],
                        vec![Token::X, Token::O, Token::Empty]];

    assert_eq!(get_diagonals(&board), expected);
}

#[test]
fn test_get_diagonals_two() {
    // works with a second 9 space board
    let board = generate_test_board_two();
    let expected = vec![vec![Token::O,     Token::X, Token::O],
                        vec![Token::Empty, Token::X, Token::X]];

    assert_eq!(get_diagonals(&board), expected);
}

#[test]
fn test_board_is_full_one() {
    // returns true if full
    let board = generate_cat_board();
    assert!(board_is_full(&board));
}

#[test]
fn test_board_is_full_two() {
    // returns false if not full
    let board = generate_test_board_one();
    assert!(!board_is_full(&board));
}

#[test]
fn test_count_tokens() {
    // returns the count of a specified token in board
    let board = generate_cat_board();
    assert_eq!(count_tokens(&board, Token::X), 5);
    assert_eq!(count_tokens(&board, Token::O), 4);
}

#[test]
fn test_empty_spaces() {
    // returns the space numers of the empty spaces in board
    let board = generate_test_board_two();
    assert_eq!(empty_spaces(&board), vec![2, 3, 5, 7]);
}

#[test]
fn test_is_empty() {
    // returns true if board contains only Token::Emptys, false otherwise
    assert!(is_empty(&generate_empty_board(3)));
    assert!(!is_empty(&generate_test_board_one()));
}
