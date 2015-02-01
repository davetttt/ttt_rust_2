use ::minimax::*;
use ::board;
use ::tests::board_tests;
use ::token::Token;

#[test]
fn test_minimax_one() {
    // with full non-won board, minimax returns (None, 0)
    let board = board_tests::generate_cat_board();
    assert_eq!(minimax(board), (None, 0));
}

#[test]
fn test_minimax_two() {
    // with O-won board, minimax returns (None, -10)
    let mut board = board::generate_empty_board(3);
    board = board_tests::set_spaces(board, vec![0, 2, 8], Token::X);
    board = board_tests::set_spaces(board, vec![1, 4, 7], Token::O);
    assert_eq!(minimax(board), (None, -10));
}

#[test]
fn test_minimax_three() {
    // with full X-won board, minimax returns (None, 10)
    let mut board = board::generate_empty_board(3);
    board = board_tests::set_spaces(board, vec![1, 2, 4, 6, 8], Token::X);
    board = board_tests::set_spaces(board, vec![0, 3, 5, 7], Token::O);
    assert_eq!(minimax(board), (None, 10));
}

#[test]
fn test_minimax_four() {
    // with winnable board and 1 open space, returns (Some(winning_space), 9)
    let mut board = board::generate_empty_board(3);
    board = board_tests::set_spaces(board, vec![1, 2, 4, 8], Token::X);
    board = board_tests::set_spaces(board, vec![0, 3, 5, 7], Token::O);
    assert_eq!(minimax(board), (Some(6), 9));
}

#[test]
fn test_minimax_five() {
    // with winnable board and 2 open spaces, returns (Some(winning_space), -8)
    let mut board = board::generate_empty_board(3);
    board = board_tests::set_spaces(board, vec![0, 2, 3, 4], Token::X);
    board = board_tests::set_spaces(board, vec![1, 6, 7], Token::O);
    assert_eq!(minimax(board), (Some(8), -9));
}

#[test]
fn test_minimax_six() {
    // with X-threatening, tie-able board, returns (Some(blocking_space), 0)
    let mut board = board::generate_empty_board(3);
    board = board_tests::set_spaces(board, vec![3, 4, 8], Token::X);
    board = board_tests::set_spaces(board, vec![1, 5], Token::O);
    assert_eq!(minimax(board), (Some(0), 0));
}

#[test]
fn test_minimax_seven() {
    // with board that can be won immediately by X,
    // returns (Some(winning_space), 9)
    let mut board = board::generate_empty_board(3);
    board = board_tests::set_spaces(board, vec![6, 8], Token::X);
    board = board_tests::set_spaces(board, vec![0, 3], Token::O);
    assert_eq!(minimax(board), (Some(7), 9));
}
