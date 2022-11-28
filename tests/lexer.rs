use test_generator::test_resources;

#[test_resources("tests/defs/*.webidl")]
fn should_lex(resource: &str) {
    let content = std::fs::read_to_string(resource).unwrap();
    let result = weedle::lexer::lex(&content);

    assert!(result.is_ok());

    let tokens = result.unwrap();

    match tokens.last().unwrap().tag {
        weedle::lexer::TokenTag::Eof => assert!(true, "Last token should be EOF"),
        _ => assert!(false, "Last token should be EOF"),
    }

    // TODO: source file reconstruction test
}
