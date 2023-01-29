use test_generator::test_resources;
use weedle::parser::eat::VariantToken;

#[test_resources("tests/defs/*.webidl")]
fn should_parse(resource: &str) {
    let content = std::fs::read_to_string(resource).unwrap();
    let result = weedle::parse(&content).unwrap();

    let resource_path = std::path::Path::new(resource);
    let stem = resource_path.file_stem().unwrap().to_str().unwrap();
    let baseline_path = std::path::Path::new("./tests/baselines/defs/").join(format!("{stem}.txt"));
    let baseline = std::fs::read_to_string(baseline_path).unwrap();

    assert_eq!(format!("{result:#?}\n"), baseline);
}

#[test_resources("tests/invalids/*.webidl")]
fn should_not_parse(resource: &str) {
    use nom::error::convert_error;
    use nom::Err::*;

    let content = std::fs::read_to_string(resource).unwrap();
    let err = weedle::parse(&content).unwrap_err();

    let resource_path = std::path::Path::new(resource);
    let stem = resource_path.file_stem().unwrap().to_str().unwrap();
    let baseline_path =
        std::path::Path::new("./tests/baselines/invalids/").join(format!("{stem}.txt"));
    let baseline = std::fs::read_to_string(baseline_path).unwrap();

    let message = match err {
        Error(e) | Failure(e) => convert_error(&content[..], e),
        Incomplete(_) => "Unexpected incomplete error".to_owned(),
    };
    assert_eq!(message, baseline);
}

#[test]
pub fn should_parse_dom_webidl() {
    let content = std::fs::read_to_string("./tests/defs/dom.webidl").unwrap();
    let parsed = weedle::parse(&content).unwrap();

    assert_eq!(parsed.len(), 62);
}

#[test]
fn should_parse_html_webidl() {
    let content = std::fs::read_to_string("./tests/defs/html.webidl").unwrap();
    let parsed = weedle::parse(&content).unwrap();

    assert_eq!(parsed.len(), 325);
}

#[test]
fn should_parse_mediacapture_streams_webidl() {
    let content = std::fs::read_to_string("./tests/defs/mediacapture-streams.webidl").unwrap();
    let parsed = weedle::parse(&content).unwrap();

    assert_eq!(parsed.len(), 37);
}

#[test]
fn should_parse_streams_webidl() {
    let content = std::fs::read_to_string("./tests/defs/streams.webidl").unwrap();
    let parsed = weedle::parse(&content).unwrap();

    assert_eq!(parsed.len(), 37);
}

#[test]
fn interface_constructor() {
    use weedle::*;

    let content = std::fs::read_to_string("./tests/defs/interface-constructor.webidl").unwrap();
    let mut parsed = weedle::parse(&content).unwrap();

    assert_eq!(parsed.len(), 1);

    let definition = parsed.pop().unwrap();

    match definition {
        Definition::Interface(mut interface) => {
            assert!(interface.attributes.is_none());
            assert_eq!(interface.interface, VariantToken::default());
            assert_eq!(interface.identifier.variant.0, "InterfaceWithConstructor");
            assert_eq!(interface.inheritance, None);

            assert_eq!(interface.members.body.len(), 1);

            let body = interface.members.body.pop().unwrap();

            match body {
                interface::InterfaceMember::Constructor(constructor) => {
                    let mut attributes = constructor.attributes.unwrap().body.list;
                    assert_eq!(attributes.len(), 1);
                    let attribute = attributes.pop().unwrap();

                    match attribute {
                        attribute::ExtendedAttribute::NoArgs(attribute) => {
                            assert_eq!((attribute.0).variant.0, "Throws");
                        }
                        _ => unreachable!(),
                    }

                    let mut args = constructor.args.body.list;
                    assert_eq!(args.len(), 1);
                    let arg = args.pop().unwrap();

                    match arg {
                        argument::Argument::Single(arg) => {
                            assert!(arg.attributes.is_none());
                            assert!(arg.optional.is_none());
                            assert!(arg.type_.attributes.is_none());

                            match arg.type_.type_ {
                                types::Type::Single(types::SingleType::Distinguishable(
                                    types::DistinguishableType::Integer(_),
                                )) => {}
                                _ => unreachable!(),
                            }
                        }
                        _ => unreachable!(),
                    };

                    assert_eq!(
                        constructor.constructor,
                        VariantToken {
                            variant: Default::default(),
                            trivia: "\n  ",
                        }
                    );
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}
