use super::token::Token;
use std::num::Float;

#[allow(unused_variables)] // i in for loop is never used
pub fn generate_empty_board(width: usize) -> Vec<Token> {
    let mut board = vec![];
    for i in range(0, width * width) {
        board.push(Token::Empty);
    }
    board
}

pub fn set_space(board: Vec<Token>, space: usize, token: Token) -> Vec<Token> {
    let mut resulting_board = board;
    resulting_board[space] = token;
    resulting_board
}

fn get_board_width(board: &Vec<Token>) -> usize {
    (board.len() as f64).sqrt() as usize
}

fn get_spaces(board: &Vec<Token>, space_numbers: Vec<usize>) -> Vec<Token> {
    let mut spaces = vec![];
    for space_number in space_numbers.iter() {
        spaces.push(board[*space_number]);
    }
    spaces
}

fn take_nth(board: &Vec<Token>, n: usize, quantity: usize) -> Vec<Token> {
    let space_numbers = range(0, board.len()).
                            filter(|&number| number % n == n || number % n == 0).
                            collect::<Vec<usize>>();
    let space_numbers_subset = space_numbers[0..quantity].to_vec();
    get_spaces(board, space_numbers_subset)
}

pub fn get_rows(board: &Vec<Token>) -> Vec<Vec<Token>> {
    let width = get_board_width(board);
    let mut rows = vec![];
    for row_number in range(0, width) {
        let board_subset = board[(row_number * width)..board.len()].to_vec();
        rows.push(take_nth(&board_subset, 1, width));
    }
    rows
}

pub fn get_columns(board: &Vec<Token>) -> Vec<Vec<Token>> {
    let width = get_board_width(board);
    let mut columns = vec![];
    for column_number in range(0, width) {
        let board_subset = board[column_number..board.len()].to_vec();
        columns.push(take_nth(&board_subset, width, width));
    }
    columns
}

fn get_downward_diagonal(board: &Vec<Token>) -> Vec<Token> {
    let width = get_board_width(board);
    take_nth(board, width + 1, width)
}

fn get_upward_diagonal(board: &Vec<Token>) -> Vec<Token> {
    let width = get_board_width(board);
    let board_subset = board[(width -1)..board.len()].to_vec();
    take_nth(&board_subset, width - 1, width)
}

pub fn get_diagonals(board: &Vec<Token>) -> Vec<Vec<Token>> {
    vec![get_downward_diagonal(board), get_upward_diagonal(board)]
}

pub fn board_is_full(board: &Vec<Token>) -> bool {
    !board.iter().any(|token| *token == Token::Empty)
}

pub fn count_tokens(board: &Vec<Token>, token: Token) -> usize {
    board.iter().filter(|&t| *t == token).count()
}

pub fn empty_spaces(board: &Vec<Token>) -> Vec<usize> {
    range(0, board.len()).
        filter(|space_number| board[*space_number] == Token::Empty).
        collect()
}

pub fn is_empty(board: &Vec<Token>) -> bool {
    board.iter().all(|token| *token == Token::Empty)
}
