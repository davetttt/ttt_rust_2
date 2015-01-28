use ::board::*;
use ::token::Token;

#[test]
fn test_generate_empty_board() {
    // with width 3, returns vector of 9 empty tokens
    let board = generate_empty_board(3);
    let expected = vec![Token::Empty, Token::Empty, Token::Empty,
                        Token::Empty, Token::Empty, Token::Empty,
                        Token::Empty, Token::Empty, Token::Empty];

    assert_eq!(board, expected);
}

