extern crate jsonxf;

#[test]
fn minimize_passes_test_cases() {
    let test_cases: Vec<[&str; 2]> = vec![
        [
            // one object, one k-v pair
            " { \"hello\": \"world\" } \r\n",
            "{\"hello\":\"world\"}",
        ],
        [
            // one object, several k-v pairs
            " { \"hello\": \"world\",\r\n  \"wow\": \"cool\"  } \r\n",
            "{\"hello\":\"world\",\"wow\":\"cool\"}",
        ],
        [
            // one object per line
            " { \"hello\": \"world\"}\r\n  { \"wow\": \"cool\"  } \r\n",
            "{\"hello\":\"world\"}\n{\"wow\":\"cool\"}",
        ],
        [
            // empty object
            " { \"hello\": {} } \r\n",
            "{\"hello\":{}}",
        ],
        [
            // nested structures
            " { \"hello\": [ \"world\" , 22 ] } \r\n",
            "{\"hello\":[\"world\",22]}",
        ],
    ];

    for case in test_cases {
        let input = case[0];
        let output = case[1];
        assert_eq!(jsonxf::minimize(input).unwrap(), output);
    }
}
