use super::token::Token;

pub fn generate_empty_board(width: usize) -> Vec<Token> {
    let mut board = vec![];
    let mut count = 0;
    loop {
        if count == width * width {
            break;
        }
        board.push(Token::Empty);
        count += 1;
    }
    board
}
