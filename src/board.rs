use super::token::Token;
use std::num::Float;

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

pub fn set_space(board: Vec<Token>, space: usize, token: Token) -> Vec<Token> {
    let mut resulting_board = board;
    resulting_board[space] = token;
    resulting_board
}

fn get_board_width(board: &Vec<Token>) -> usize {
    (board.len() as f64).sqrt() as usize
}

pub fn get_rows(board: Vec<Token>) -> Vec<Vec<Token>> {
    let width = get_board_width(&board);
    let mut rows = vec![];
    for row_number in range(0, width) {
        let row_start = row_number * width;
        let row = board[row_start..(row_start + width)].to_vec();
        rows.push(row);
    }
    rows
}

fn get_spaces(board: &Vec<Token>, space_numbers: Vec<usize>) -> Vec<Token> {
    let mut spaces = vec![];
    for space_number in space_numbers.iter() {
        spaces.push(board[*space_number]);
    }
    spaces
}

fn get_column(board: &Vec<Token>, column_number: usize) -> Vec<Token> {
    let width = get_board_width(board);
    let board_subset = board[column_number..board.len()].to_vec();
    let space_numbers = range(0, board_subset.len()).
                            filter(|&x| x % width == width || x % width == 0).
                            collect::<Vec<usize>>();
    get_spaces(&board_subset, space_numbers)
}

pub fn get_columns(board: Vec<Token>) -> Vec<Vec<Token>> {
    let width = get_board_width(&board);
    let mut columns = vec![];
    for column_number in range(0, width) {
        columns.push(get_column(&board, column_number));
    }
    columns
}
