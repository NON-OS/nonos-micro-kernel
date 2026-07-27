extern crate jsonxf;
use jsonxf::Formatter;

#[test]
fn indent() {
    let mut xf = Formatter::minimizer();
    xf.indent = String::from("X");
    assert_eq!(
        "{X\"a\":{XX\"b\":{XXX\"c\":3XX}X}}",
        xf.format("{\"a\":{\"b\":{\"c\":3}}}").unwrap()
    );
}

#[test]
fn line_separator() {
    let mut xf = Formatter::minimizer();
    xf.line_separator = String::from("X");
    assert_eq!(
        "{X\"a\":{X\"b\":{X\"c\":3X}X}X}",
        xf.format("{\"a\":{\"b\":{\"c\":3}}}").unwrap()
    );
}

#[test]
fn record_separator() {
    let mut xf = Formatter::minimizer();
    xf.record_separator = String::from("X");
    assert_eq!(
        "{\"a\":{\"b\":{\"c\":3}}}X{\"a\":{\"b\":{\"c\":3}}}",
        xf.format("{\"a\":{\"b\":{\"c\":3}}}{\"a\":{\"b\":{\"c\":3}}}")
            .unwrap()
    );
}

#[test]
fn after_colon() {
    let mut xf = Formatter::minimizer();
    xf.after_colon = String::from("X");
    assert_eq!(
        "{\"a\":X{\"b\":X{\"c\":X3}}}",
        xf.format("{\"a\":{\"b\":{\"c\":3}}}").unwrap()
    );
}

#[test]
fn trailing_output() {
    let mut xf = Formatter::minimizer();
    xf.trailing_output = String::from("X");
    assert_eq!(
        "{\"a\":{\"b\":{\"c\":3}}}X",
        xf.format("{\"a\":{\"b\":{\"c\":3}}}").unwrap()
    );
}
