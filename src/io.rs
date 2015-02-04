use std::io::stdin;
use std::io::BufferedReader;
use super::token::Token;
use super::board;

pub trait Io {
    fn prompt_with_options(&self, question: &str, options: Vec<&str>) -> usize;
    fn display_board(&self, board: &Vec<Token>);
    fn print(&self, text: &str);
}

pub struct TestIo {
    answer: String,
}

impl TestIo {
    pub fn new(answer: String) -> TestIo {
        TestIo { answer: answer }
    }
}

impl Io for TestIo {
    #[allow(unused_variables)] // TestIo doesn't use questions that are passed in
    fn prompt_with_options(&self, question: &str, options: Vec<&str>) -> usize {
        match options.iter().position(|&option| option == self.answer) {
            Some(chosen_option_index) => chosen_option_index,
            None => panic!("TestIo chose an invalid option"),
        }
    }
    fn display_board(&self, board: &Vec<Token>) {}
    fn print(&self, text: &str) {}
}

pub struct ConsoleIo;

impl ConsoleIo {
    pub fn new() -> ConsoleIo {
        ConsoleIo
    }

    fn format_space(&self, token: Token, space_number: usize) -> String {
        let space_number_string = space_number.to_string();
        match token {
            Token::X => "X ".to_string(),
            Token::O => "O ".to_string(),
            _        => format!("{} ", space_number_string),
        }
    }

    fn format_row(&self, row: &Vec<Token>, row_number: usize) -> String {
        let mut result: String = "".to_string();
        for i in range(0, row.len()) {
            let space_number = row_number + i;
            result.push_str(self.format_space(row[i], space_number).as_slice());
        }
        result.push_str("\n");
        result
    }
}

impl Io for ConsoleIo {
    fn prompt_with_options(&self, question: &str, options: Vec<&str>) -> usize {
        print!("{}", question);
        let answer = match BufferedReader::new(stdin()).read_line() {
            Ok(line) => line,
            Err(e)   => panic!(e),
        };
        match options.iter().
                  position(|option| option.as_slice() == answer.trim()) {
            Some(chosen_option_index) => chosen_option_index,
            None => self.prompt_with_options(question, options),
        }
    }

    fn display_board(&self, board: &Vec<Token>) {
        let rows = board::get_rows(board);
        let mut result: String = "".to_string();
        for row_number in range(0, rows.len()) {
            result.push_str(
                self.format_row(&rows[row_number], row_number).as_slice());
        }
        print!("{}", result.as_slice())
    }

    fn print(&self, text: &str) {
        print!("{}", text)
    }
}
