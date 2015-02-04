use ::io::*;

#[test]
fn test_test_io_prompt_with_options_one() {
    // TestIo gets initialized with an answer,
    // and uses that answer when prompt_with_options is called.
    // prompt_with_options returns the index of the answer
    // in the list of passed-in options
    let io = TestIo::new("three".to_string());
    let options = vec!["one", "two", "three", "four"];
    assert_eq!(io.prompt_with_options("?", options), 2);
}

