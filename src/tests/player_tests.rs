use ::player::*;
use ::board;
use ::token::Token;
use ::tests::board_tests;

#[test]
fn test_cpu_player_make_move_one() {
    // with empty 3x3 board, returns board with top left space taken
    let cpu_player = CpuPlayer::new(Token::X);
    let board = board::generate_empty_board(3);

    assert_eq!(cpu_player.make_move(board.clone()),
               board::set_space(board, 0, Token::X));
}

#[test]
fn test_cpu_player_make_move_two() {
    // with board that can be won immediately, returns won board
    let cpu_player = CpuPlayer::new(Token::X);
    let mut board = board::generate_empty_board(3);
    board = board_tests::set_spaces(board, vec![6, 8], Token::X);
    board = board_tests::set_spaces(board, vec![0, 3], Token::O);

    assert_eq!(cpu_player.make_move(board.clone()),
               board::set_space(board, 7, Token::X));
}

#[test]
fn test_cpu_player_make_move_three() {
    // with near-won board, returns a board that blocks the threat
    let cpu_player = CpuPlayer::new(Token::O);
    let mut board = board::generate_empty_board(3);
    board = board_tests::set_spaces(board, vec![3, 4, 7], Token::X);
    board = board_tests::set_spaces(board, vec![1, 6], Token::O);

    assert_eq!(cpu_player.make_move(board.clone()),
               board::set_space(board, 5, Token::O));
}

