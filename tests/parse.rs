use tumekiri::{JsonParser, JsonValue};

#[test]
fn parse_string_test() {
    let input = r#""string""#;
    let parse_result = JsonParser::new(input.chars()).parse();

    assert!(parse_result.is_ok());

    let value = parse_result.unwrap();

    assert_eq!(value, JsonValue::String("string".to_string()));
}

#[test]
fn parse_number_test() {
    let in_out = vec![
        ("111", 111.0),
        ("1.0", 1.0),
        ("0.0", 0.0),
        ("1e12", 1e12),
        ("-55", -55.0),
        ("-55.5", -55.5),
        ("-1e12", -1e12),
    ];

    for (input, out) in in_out {
        let parse_result = JsonParser::new(input.chars()).parse();

        assert!(parse_result.is_ok());

        let value = parse_result.unwrap();

        assert_eq!(value, JsonValue::Number(out));
    }
}
