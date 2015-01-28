pub trait Io {
    fn prompt(&mut self, question: String) -> String;
}

pub struct TestIo {
    answers: Vec<String>,
    prompt_count: usize,
}

impl TestIo {
    pub fn new(injected_answers: Vec<String>) -> TestIo {
        TestIo {
            answers: injected_answers,
            prompt_count: 0,
        }
    }
}

impl Io for TestIo {
    #[allow(unused_variables)] // TestIo doesn't care about question
    fn prompt(&mut self, question: String) -> String {
        self.prompt_count += 1;
        self.answers[self.prompt_count - 1].clone()
    }
}
