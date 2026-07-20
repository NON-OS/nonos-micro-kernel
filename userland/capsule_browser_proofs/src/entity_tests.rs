// NONOS Operating System (AGPL-3.0-or-later)
//! Known-answer proofs for HTML entity decoding: named entities, decimal and
//! hex numeric references, and the pass-through of unknown/invalid ones. Text
//! carries entities on every page; a wrong decode corrupts visible content.

use crate::browser::html::entity::push_decoded;

fn dec(e: &str) -> String {
    let mut s = String::new();
    push_decoded(&mut s, e);
    s
}

#[test]
fn named_entities_decode() {
    assert_eq!(dec("amp"), "&");
    assert_eq!(dec("lt"), "<");
    assert_eq!(dec("gt"), ">");
    assert_eq!(dec("quot"), "\"");
    assert_eq!(dec("apos"), "'");
    assert_eq!(dec("nbsp"), " ");
}

#[test]
fn numeric_entities_decimal_and_hex() {
    assert_eq!(dec("#65"), "A", "decimal 65 is 'A'");
    assert_eq!(dec("#x41"), "A", "hex 41 is 'A'");
    assert_eq!(dec("#X41"), "A", "uppercase X prefix");
    assert_eq!(dec("#8364"), "\u{20AC}", "decimal 8364 is the euro sign");
    assert_eq!(dec("#x20AC"), "\u{20AC}", "hex 20AC is the euro sign");
}

#[test]
fn unknown_and_invalid_entities_pass_through_literally() {
    assert_eq!(dec("bogus"), "&bogus;", "an unknown name stays literal");
    assert_eq!(dec("#xZZ"), "&#xZZ;", "invalid hex stays literal");
    assert_eq!(dec("#0"), "&#0;", "a control code is not emitted");
}
