use tumekiri::{JsonParser, JsonValue};

#[test]
fn parse_string_test() {
    let input = r#""string""#;
    let parse_result = JsonParser::new(input.chars()).parse();

    assert!(parse_result.is_ok());

    let value = parse_result.unwrap();

    assert_eq!(value, JsonValue::String("string".to_string()));
}
