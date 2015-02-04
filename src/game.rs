use super::token::Token;
use super::player;
use super::rules;
use super::io::Io;

pub fn game_loop(players: &Vec<Box<player::Player>>,
                 board: &Vec<Token>) -> Option<Token> {
    if rules::game_is_over(board) {
        rules::get_winner(board)
    } else {
        let token = rules::get_turn_token(board);
        let player = match players.iter().find(|p| p.get_token() == token) {
            Some(x) => x,
            None    => panic!("Problem determining turn"),
        };
        let board_after_move = player.make_move(board);
        game_loop(players, &board_after_move)
    }
}

// this method is not tested, so it generates a warning
// whenever the tests are run
#[allow(dead_code)]
pub fn play_game<I: Io>(players: &Vec<Box<player::Player>>,
                        board: &Vec<Token>,
                        io: &I) {
    let winning_token = game_loop(players, board);
    let message = match winning_token {
        Some(Token::X) => "X wins!\n",
        Some(Token::O) => "O wins!\n",
        _              => "Draw.\n",
    };
    io.print(message);
}
