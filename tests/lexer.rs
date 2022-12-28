use test_generator::test_resources;

#[test_resources("tests/defs/*.webidl")]
fn should_lex(resource: &str) {
    let content = std::fs::read_to_string(resource).unwrap();
    let tokens = weedle::lexer::lex(&content).unwrap();

    assert!(
        matches!(tokens.last().unwrap().tag, weedle::lexer::Tag::Eof(_)),
        "Last token should be EOF"
    );

    // TODO: source file reconstruction test
}
