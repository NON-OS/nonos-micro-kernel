extern crate jsonxf;

#[test]
fn pretty_print_passes_test_cases() {
    let test_cases: Vec<[&str; 2]> = vec![
        [
            // one object, one k-v pair
            "{\"hello\":\"world\"}",
            "{\n  \"hello\": \"world\"\n}",
        ],
        [
            // one object, several k-v pairs
            " { \"hello\": \"world2\",\r\n  \"wow\": \"cool\"  } \r\n",
            "{\n  \"hello\": \"world2\",\n  \"wow\": \"cool\"\n}",
        ],
        [
            // simple array
            "[1,2,3]",
            "[\n  1,\n  2,\n  3\n]",
        ],
        [
            // one object per line, with blank lines and missing newlines
            " { \"hello\": \"world3\"}\r\n\n\n  { \"wow\": \"cool\"  }{\"a\":\"b\"}",
            "{\n  \"hello\": \"world3\"\n}\n{\n  \"wow\": \"cool\"\n}\n{\n  \"a\": \"b\"\n}",
        ],
        [
            // one array per line, with blank lines and missing newlines
            "[1, 2, \"omg\"]\n\n\n[\"whee\", {}, 22][]",
            "[\n  1,\n  2,\n  \"omg\"\n]\n[\n  \"whee\",\n  {},\n  22\n]\n[]",
        ],
        [
            // nested empty array
            " { \"hello\": [\n\n] }",
            "{\n  \"hello\": []\n}",
        ],
        [
            // nested empty object
            " { \"hello\": {} }",
            "{\n  \"hello\": {}\n}",
        ],
        [
            // nested structures
            " { \"hello\": [ \"world5\" , 22 ] } \r\n",
            "{\n  \"hello\": [\n    \"world5\",\n    22\n  ]\n}",
        ],
        [
            // empty object special case
            " { \n\r\n\t } ",
            "{}",
        ],
    ];

    for case in test_cases {
        let input = case[0];
        let output = case[1];
        assert_eq!(jsonxf::pretty_print(input).unwrap(), output);
    }
}

#[test]
fn eager_record_separators() {
    let test_cases = vec![
        ("{1: 1}{}", "{\n  1: 1\n}\n{}\n"),
        ("{1: 1}{1: 1}", "{\n  1: 1\n}\n{\n  1: 1\n}\n"),
        ("[]", "[]\n"),
    ];
    for (input, output) in test_cases {
        let mut printer = jsonxf::Formatter::pretty_printer();
        printer.eager_record_separators = true;
        assert_eq!(printer.format(input).unwrap(), output);
    }
}
