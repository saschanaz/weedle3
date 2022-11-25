use test_generator::test_resources;

#[test_resources("tests/defs/*.webidl")]
fn should_parse(resource: &str) {
    let content = std::fs::read_to_string(resource).unwrap();
    let parsed = weedle::parse(&content);

    assert!(parsed.is_ok());
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
            assert_eq!(interface.interface, term!(interface));
            assert_eq!(interface.identifier.0, "InterfaceWithConstructor");
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
                            assert_eq!((attribute.0).0, "Throws");
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
                                types::Type::Single(types::SingleType::NonAny(
                                    types::NonAnyType::Integer(_),
                                )) => {}
                                _ => unreachable!(),
                            }
                        }
                        _ => unreachable!(),
                    };

                    assert_eq!(constructor.constructor, term::Constructor);
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}
