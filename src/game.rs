use super::token::Token;
use super::player;
use super::rules;

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

