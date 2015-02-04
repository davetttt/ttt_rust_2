use super::board;
use super::token::Token;
use super::minimax;
use super::io::Io;

pub trait Player {
    fn get_token(&self) -> Token;
    fn make_move(&self, board: &Vec<Token>) -> Vec<Token>;
}

pub struct HumanPlayer<I: Io> {
    io: I,
    token: Token,
}

impl<I: Io> HumanPlayer<I> {
    pub fn new(io: I, token: Token) -> HumanPlayer<I> {
        HumanPlayer { io: io, token: token }
    }
}

impl<I: Io> Player for HumanPlayer<I> {
    fn get_token(&self) -> Token {
        self.token
    }

    fn make_move(&self, board: &Vec<Token>) -> Vec<Token> {
        let empty_spaces = board::empty_spaces(board);
        let empty_space_strings: Vec<String> = empty_spaces.iter().
                map(|num| num.to_string()).collect();
        let options = empty_space_strings.iter().map(|s| s.as_slice()).collect();
        let chosen_option = self.io.prompt_with_options("Enter move: ", options);

        board::set_space(board, empty_spaces[chosen_option], self.token)
    }
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
