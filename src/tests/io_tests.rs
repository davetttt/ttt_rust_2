use ::io::*;

#[test]
#[allow(unused_variables)] // i in for loop is never used
fn test_test_io_prompt() {
    // prompting a TestIo returns its injected_answers, one after another
    let injected_answers = vec!["invalid".to_string(),
                                "also invalid".to_string(),
                                "correct".to_string()];
    let mut io = TestIo::new(injected_answers.clone());
    let mut responses = vec![];

    for i in range(0, injected_answers.len()) {
        responses.push(io.prompt("".to_string()));
    }

    assert_eq!(responses, injected_answers);
}
