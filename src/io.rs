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

    #[allow(unused_variables)]
    pub fn prompt(&mut self, question: String) -> String {
        self.prompt_count += 1;
        self.answers[self.prompt_count - 1].clone()
    }
}
