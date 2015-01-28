use super::board;
use super::token::Token;

pub struct CpuPlayer {
    token: Token
}

impl CpuPlayer {
    pub fn new(token: Token) -> CpuPlayer {
        CpuPlayer { token: token }
    }

    pub fn make_move(&self, board: Vec<Token>) -> Vec<Token> {
        board::set_space(board, 0, self.token)
    }
}
