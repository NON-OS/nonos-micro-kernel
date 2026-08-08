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
    // The character it names, not a plain space. Layout may break at a
    // space and must not break at this one, so folding them together undoes
    // what an author wrote it for.
    assert_eq!(dec("nbsp"), "\u{00A0}");
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

#[test]
fn typography_decodes_to_characters_not_lookalikes() {
    // These used to fold onto ASCII, so an em dash arrived as two hyphens
    // and changed where every following line wrapped.
    assert_eq!(dec("mdash"), "\u{2014}");
    assert_eq!(dec("ndash"), "\u{2013}");
    assert_eq!(dec("hellip"), "\u{2026}");
    assert_eq!(dec("lsquo"), "\u{2018}");
    assert_eq!(dec("rsquo"), "\u{2019}");
    assert_eq!(dec("ldquo"), "\u{201C}");
    assert_eq!(dec("rdquo"), "\u{201D}");
    assert_eq!(dec("copy"), "\u{00A9}");
    assert_eq!(dec("reg"), "\u{00AE}");
    assert_eq!(dec("trade"), "\u{2122}");
    assert_eq!(dec("times"), "\u{00D7}");
    assert_eq!(dec("laquo"), "\u{00AB}");
}

#[test]
fn accented_latin_decodes() {
    // Any page not in English carries these, and one that arrives undecoded
    // prints its own source at the reader.
    assert_eq!(dec("ccedil"), "\u{00E7}");
    assert_eq!(dec("eacute"), "\u{00E9}");
    assert_eq!(dec("uuml"), "\u{00FC}");
    assert_eq!(dec("ntilde"), "\u{00F1}");
    assert_eq!(dec("szlig"), "\u{00DF}");
    assert_eq!(dec("oslash"), "\u{00F8}");
    assert_eq!(dec("Eacute"), "\u{00C9}");
    assert_eq!(dec("AElig"), "\u{00C6}");
}

#[test]
fn currency_arrows_and_greek_decode() {
    assert_eq!(dec("euro"), "\u{20AC}");
    assert_eq!(dec("pound"), "\u{00A3}");
    assert_eq!(dec("rarr"), "\u{2192}");
    assert_eq!(dec("le"), "\u{2264}");
    assert_eq!(dec("alpha"), "\u{03B1}");
    assert_eq!(dec("Omega"), "\u{03A9}");
}

#[test]
fn windows_code_page_numbers_are_read_as_meant() {
    // Pages still carry numbers written for the old code page, where these
    // name control positions that were never those characters.
    assert_eq!(dec("#151"), "\u{2014}");
    assert_eq!(dec("#146"), "\u{2019}");
    assert_eq!(dec("#128"), "\u{20AC}");
}

#[test]
fn an_unknown_reference_is_written_back_out() {
    // Better to show the source than to drop the text around it.
    assert_eq!(dec("notarealentity"), "&notarealentity;");
}
