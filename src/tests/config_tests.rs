use ::config::*;
use ::io::*;
use ::player::*;
use ::token::Token;

#[test]
fn test_configure_players_one() {
    // with answer = "1", returns vec![human(X), cpu(O)]
    // This test only tests the vector length and token values
    // because Show must be implemented in order to check for struct equality
    // and it is difficult (impossible?) to implement show for a trait
    // (configure_players returns a vector of Players, Player is a trait)
    let io = TestIo::new("1".to_string());
    let mut expected: Vec<Box<Player>> = Vec::new();
    expected.push(Box::new(HumanPlayer::new(ConsoleIo::new(), Token::X)));
    expected.push(Box::new(CpuPlayer::new(Token::O)));
    let players = configure_players(&io);

    assert_eq!(players.len(), 2);
    assert_eq!(players[0].get_token(), Token::X);
    assert_eq!(players[1].get_token(), Token::O);
//    assert_eq!(configure_players(&io), expected);
}

#[test]
fn test_configure_players_two() {
    // with answer = "2", returns vec![cpu(X), human(O)]
    // This test only tests the vector length and token values
    // See test_configure_players_one for explanation
    let io = TestIo::new("2".to_string());
    let mut expected: Vec<Box<Player>> = Vec::new();
    expected.push(Box::new(CpuPlayer::new(Token::X)));
    expected.push(Box::new(HumanPlayer::new(ConsoleIo::new(), Token::O)));
    let players = configure_players(&io);

    assert_eq!(players.len(), 2);
    assert_eq!(players[0].get_token(), Token::X);
    assert_eq!(players[1].get_token(), Token::O);
//    assert_eq!(configure_players(&io), expected);
}

#[test]
fn test_configure_players_three() {
    // with answer = "3", returns vec![human(X), human(O)]
    // This test only tests the vector length and token values
    // See test_configure_players_one for explanation
    let io = TestIo::new("3".to_string());
    let mut expected: Vec<Box<Player>> = Vec::new();
    expected.push(Box::new(HumanPlayer::new(ConsoleIo::new(), Token::X)));
    expected.push(Box::new(HumanPlayer::new(ConsoleIo::new(), Token::O)));
    let players = configure_players(&io);

    assert_eq!(players.len(), 2);
    assert_eq!(players[0].get_token(), Token::X);
    assert_eq!(players[1].get_token(), Token::O);
//    assert_eq!(configure_players(&io), expected);
}

#[test]
fn test_play_again_one() {
    // with answer = "y", returns true
    let io = TestIo::new("y".to_string());
    assert!(play_again(&io));
}

#[test]
fn test_play_again_two() {
    // with answer = "n", returns false
    let io = TestIo::new("n".to_string());
    assert!(!play_again(&io));
}

