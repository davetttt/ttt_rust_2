use super::board;
use super::token::Token;
use super::minimax;

pub trait Player {
    fn get_token(&self) -> Token;
    fn make_move(&self, board: &Vec<Token>) -> Vec<Token>;
}

pub struct CpuPlayer {
    token: Token
}

impl CpuPlayer {
    pub fn new(token: Token) -> CpuPlayer {
        CpuPlayer { token: token }
    }
}

impl Player for CpuPlayer {
    fn get_token(&self) -> Token {
        self.token
    }

    fn make_move(&self, board: &Vec<Token>) -> Vec<Token> {
        // minimax with an empty board is slow
        // so space number is hardcoded to 0 in that case
        let space = match board::is_empty(board) {
            true  => 0,
            false => match minimax::minimax(board).0 {
                         Some(x) => x,
                         None    => panic!("Minimax did not return a space"),
            },
        };
        board::set_space(board, space, self.token)
    }
}
