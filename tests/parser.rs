use test_generator::test_resources;

#[test_resources("tests/defs/constants.webidl")]
fn should_parse(resource: &str) {
    let content = std::fs::read_to_string(resource).unwrap();
    let defs = weedle::parser::parse(&content).unwrap();

    assert!(
        matches!(defs.last().unwrap(), weedle::parser::Definition::Eof(_)),
        "Last token should be EOF"
    );

    // TODO: source file reconstruction test
}
