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

#[test]
fn parse_array_test() {
    let in_out = vec![
        (
            "[1,2,3  ,4,   5]",
            JsonValue::Array(vec![
                JsonValue::Number(1.0),
                JsonValue::Number(2.0),
                JsonValue::Number(3.0),
                JsonValue::Number(4.0),
                JsonValue::Number(5.0),
            ]),
        ),
        (
            r#"[1,2,3, [], [1, "string"]]"#,
            JsonValue::Array(vec![
                JsonValue::Number(1.0),
                JsonValue::Number(2.0),
                JsonValue::Number(3.0),
                JsonValue::Array(vec![]),
                JsonValue::Array(vec![
                    JsonValue::Number(1.0),
                    JsonValue::String("string".to_string()),
                ]),
            ]),
        ),
    ];

    for (input, out) in in_out {
        let value = JsonParser::new(input.chars()).parse().unwrap();

        assert_eq!(value, out);
    }
}
