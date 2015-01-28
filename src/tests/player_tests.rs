use ::player::*;
use ::board;
use ::token::Token;

#[test]
fn test_cpu_player_make_move_one() {
    // CpuPlayer chooses the top left space in an empty 3x3 board
    let cpu_player = CpuPlayer::new(Token::X);
    let empty_board = board::generate_empty_board(3);
    let expected = board::set_space(empty_board.clone(), 0, Token::X);

    assert_eq!(cpu_player.make_move(empty_board), expected);
}
