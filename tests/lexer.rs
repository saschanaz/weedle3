use test_generator::test_resources;

#[test_resources("tests/defs/*.webidl")]
fn should_lex(resource: &str) {
    let content = std::fs::read_to_string(resource).unwrap();
    let result = weedle::lexer::tokens(&content);

    assert!(result.is_ok());

    let (unread, tokens) = result.unwrap();
    assert!(unread.is_empty());

    match tokens.last().unwrap().tag {
        weedle::lexer::TokenTag::Eof => assert!(true, "Last token should be EOF"),
        _ => assert!(false, "Last token should be EOF"),
    }

    // TODO: source file reconstruction test
}
