pub trait Io {
    fn prompt_with_options(&self, question: &str, options: Vec<&str>) -> usize;
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
    fn prompt_with_options(&self, question: &str, options: Vec<&str>) -> usize {
        match options.iter().position(|&option| option == self.answer) {
            Some(chosen_option_index) => chosen_option_index,
            None => panic!("TestIo chose an invalid option"),
        }
    }
}

pub struct ConsoleIo;

impl ConsoleIo {
    pub fn new() -> ConsoleIo {
        ConsoleIo
    }
}

impl Io for ConsoleIo {
    fn prompt_with_options(&self, question: &str, options: Vec<&str>) -> usize {
        0
    }
}
