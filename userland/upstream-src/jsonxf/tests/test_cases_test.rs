extern crate jsonxf;
use std::fs::File;
use std::io::Read;

fn test(name: &str) {
    let mut input = String::new();
    File::open(format!("./tests/test_cases/{}.json", name))
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let mut pretty = String::new();
    File::open(format!("./tests/test_cases/{}.pretty.json", name))
        .unwrap()
        .read_to_string(&mut pretty)
        .unwrap();

    let mut min = String::new();
    File::open(format!("./tests/test_cases/{}.min.json", name))
        .unwrap()
        .read_to_string(&mut min)
        .unwrap();

    assert_eq!(pretty, jsonxf::pretty_print(&input).unwrap());
    assert_eq!(min, jsonxf::minimize(&input).unwrap());
}

#[test]
fn backslash_string_test() {
    test("backslash-string");
}

#[test]
fn empty_list_test() {
    test("empty-list");
}

#[test]
fn empty_nest_test() {
    test("empty-nest");
}

#[test]
fn empty_object_test() {
    test("empty-object");
}

#[test]
fn multiple_objects_test() {
    test("multiple-objects");
}

#[test]
fn simple_list_test() {
    test("simple-list");
}

#[test]
fn simple_object_test() {
    test("simple-object");
}

#[test]
fn loose_bracket() {
    test("loose-bracket");
}
