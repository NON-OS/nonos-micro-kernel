// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// Host-side harness for the terminal's UTF-8 decoder. Compiled with the host
// toolchain so the byte-to-character logic is exercised without QEMU:
//   rustc --edition 2021 --test tests/utf8_host.rs -o /tmp/utf8_host && /tmp/utf8_host
//
// The module is pulled in by path so this cannot pass against a copy that has
// drifted from what the capsule runs.

#[path = "../src/term/vt/utf8/mod.rs"]
mod utf8;

use utf8::Utf8;

/// Feed a whole string and collect what comes back out.
fn decode(bytes: &[u8]) -> String {
    let mut u = Utf8::default();
    let mut out = String::new();
    for &b in bytes {
        u.push(b, |ch| out.push(ch));
    }
    out
}

#[test]
fn ascii_passes_through_unchanged() {
    assert_eq!(decode(b"hello world"), "hello world");
}

/// The reason this exists. A character above ASCII arrives as several bytes,
/// and printing each one turns it into that many pieces of mojibake.
#[test]
fn multibyte_characters_survive() {
    for s in
        ["\u{00D8}", "caf\u{00E9}", "\u{4F60}\u{597D}", "\u{1F512}", "\u{2502}\u{2500}\u{250C}"]
    {
        assert_eq!(decode(s.as_bytes()), s, "failed on {s}");
    }
}

/// Output arrives in whatever chunks the reader hands over, so a character
/// can be split across two feeds and the decoder has to carry state.
#[test]
fn a_character_split_across_feeds_survives() {
    let s = "\u{4F60}\u{597D}";
    let bytes = s.as_bytes();
    for split in 1..bytes.len() {
        let mut u = Utf8::default();
        let mut out = String::new();
        for &b in &bytes[..split] {
            u.push(b, |ch| out.push(ch));
        }
        for &b in &bytes[split..] {
            u.push(b, |ch| out.push(ch));
        }
        assert_eq!(out, s, "split at {split}");
    }
}

/// An overlong encoding decodes to the same value as a shorter one and exists
/// only to slip past a check that reads the short form.
#[test]
fn overlong_encodings_are_refused() {
    // '/' written as two bytes, and as three.
    assert_eq!(decode(&[0xC0, 0xAF]), "\u{FFFD}");
    assert_eq!(decode(&[0xE0, 0x80, 0xAF]), "\u{FFFD}");
    // NUL written long.
    assert_eq!(decode(&[0xC0, 0x80]), "\u{FFFD}");
}

/// Surrogate halves are not characters. They encode cleanly in UTF-8 but
/// name nothing.
#[test]
fn surrogates_are_refused() {
    assert_eq!(decode(&[0xED, 0xA0, 0x80]), "\u{FFFD}");
}

/// Past the top of the Unicode range there is nothing to name.
#[test]
fn out_of_range_is_refused() {
    // One for the lead byte, then one for each continuation with nothing to
    // continue, which is what a standard decoder reports.
    assert_eq!(decode(&[0xF5, 0x80, 0x80, 0x80]), "\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}");
    assert_eq!(decode(&[0xFF]), "\u{FFFD}");
}

/// A continuation byte with nothing to continue is not a character.
#[test]
fn a_stray_continuation_is_refused() {
    assert_eq!(decode(&[0x80]), "\u{FFFD}");
}

/// A sequence cut short must not swallow the byte that follows it. That byte
/// is far more likely to start a valid character than to belong to the
/// broken one.
#[test]
fn a_truncated_sequence_does_not_eat_the_next_character() {
    // Two bytes of a three byte character, then a plain 'A'.
    assert_eq!(decode(&[0xE4, 0xBD, b'A']), "\u{FFFD}A");
}

/// A malformed sequence costs one column, not zero, so what is on screen
/// still lines up with what was sent.
#[test]
fn every_bad_sequence_yields_exactly_one_character() {
    assert_eq!(decode(&[0x80, 0x80, 0x80]).chars().count(), 3);
}
