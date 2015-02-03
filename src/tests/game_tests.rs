use ::board;
use ::tests::board_tests;
use ::player::CpuPlayer;
use ::player::Player;
use ::token::Token;
use ::game::*;

#[test]
fn test_game_loop_one() {
    // with 2 CpuPlayers and empty board, returns None (draw)
    let mut players: Vec<Box<Player>> = Vec::new();
    players.push(Box::new(CpuPlayer::new(Token::X)));
    players.push(Box::new(CpuPlayer::new(Token::O)));
    let winner = game_loop(&players, &board::generate_empty_board(3));

    assert!(winner.is_none());
}

#[test]
fn test_game_loop_two() {
    // with 2 CpuPlayers and board that can be won by X, returns X
    let mut players: Vec<Box<Player>> = Vec::new();
    players.push(Box::new(CpuPlayer::new(Token::X)));
    players.push(Box::new(CpuPlayer::new(Token::O)));

    let mut board = board::generate_empty_board(3);
    board = board_tests::set_spaces(board, vec![0, 2, 8], Token::X);
    board = board_tests::set_spaces(board, vec![4, 6], Token::O);

    let winner = game_loop(&players, &board);

    assert!(winner.is_some());
    assert_eq!(winner.unwrap(), Token::X);
}

#[test]
fn test_game_loop_three() {
    // with 2 CpuPlayers and board that can be won immediately by O, returns O
    let mut players: Vec<Box<Player>> = Vec::new();
    players.push(Box::new(CpuPlayer::new(Token::X)));
    players.push(Box::new(CpuPlayer::new(Token::O)));

    let mut board = board::generate_empty_board(3);
    board = board_tests::set_spaces(board, vec![2, 3, 6], Token::X);
    board = board_tests::set_spaces(board, vec![1, 4], Token::O);

    let winner = game_loop(&players, &board);

    assert!(winner.is_some());
    assert_eq!(winner.unwrap(), Token::O);
}
