use ::rules::*;
use ::board;
use ::tests::board_tests;
use ::token::Token;

#[test]
fn test_game_is_over_one() {
    // with won board, returns true
    let mut board = board::generate_empty_board(3);
    board = board_tests::set_spaces(board, vec![0, 2, 4, 8], Token::X);
    board = board_tests::set_spaces(board, vec![1, 6, 7], Token::O);
    assert!(game_is_over(&board));
}

#[test]
fn test_game_is_over_two() {
    // with full board, returns true
    let mut board = board::generate_empty_board(3);
    board = board_tests::set_spaces(board, vec![0, 1, 5, 6, 8], Token::X);
    board = board_tests::set_spaces(board, vec![2, 3, 4, 7], Token::O);
    assert!(game_is_over(&board));
}

#[test]
fn test_game_is_over_three() {
    // with non-full non-won board, returns false
    let mut board = board::generate_empty_board(3);
    board = board_tests::set_spaces(board, vec![0, 1, 5, 6], Token::X);
    board = board_tests::set_spaces(board, vec![2, 3, 4, 7], Token::O);
    assert!(!game_is_over(&board));
}
