use ::minimax::*;
use ::board;
use ::tests::board_tests;
use ::token::Token;

#[test]
fn test_minimax_one() {
    // with full non-won board, minimax returns (None, 0)
    let board = board_tests::generate_cat_board();
    let space_score_tuple = minimax(board);
    let space = match space_score_tuple.0 {
        Some(x) => x,
        None    => 999
    };
    let score = space_score_tuple.1;

    assert_eq!(space, 999);
    assert_eq!(score, 0);
}

#[test]
fn test_minimax_two() {
    // with O-won board, minimax returns (None, -10)
    let mut board = board::generate_empty_board(3);
    board = board_tests::set_spaces(board, vec![0, 2, 8], Token::X);
    board = board_tests::set_spaces(board, vec![1, 4, 7], Token::O);
    let space_score_tuple = minimax(board);
    let space = match space_score_tuple.0 {
        Some(x) => x,
        None    => 999
    };
    let score = space_score_tuple.1;

    assert_eq!(space, 999);
    assert_eq!(score, -10);
}

#[test]
fn test_minimax_three() {
    // with full X-won board, minimax returns (None, 10)
    let mut board = board::generate_empty_board(3);
    board = board_tests::set_spaces(board, vec![1, 2, 4, 6, 8], Token::X);
    board = board_tests::set_spaces(board, vec![0, 3, 5, 7], Token::O);
    let space_score_tuple = minimax(board);
    let space = match space_score_tuple.0 {
        Some(x) => x,
        None    => 999
    };
    let score = space_score_tuple.1;

    assert_eq!(space, 999);
    assert_eq!(score, 10);
}

