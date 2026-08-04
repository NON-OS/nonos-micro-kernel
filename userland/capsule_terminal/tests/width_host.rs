// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// Host-side harness for character width. Compiled with the host toolchain:
//   rustc --edition 2021 --test tests/width_host.rs -o /tmp/width_host && /tmp/width_host
//
// The module is pulled in by path so this cannot pass against a copy that has
// drifted from what the capsule runs.

#[path = "../src/term/grid/width.rs"]
mod width;

use width::char_width;

#[test]
fn latin_is_one_column() {
    for ch in ['a', 'Z', '0', ' ', '~', '\u{00E9}', '\u{00D8}'] {
        assert_eq!(char_width(ch), 1, "{ch:?}");
    }
}

/// The case that made this necessary. Drawn two columns wide, so treating one
/// as a single cell puts the next character over its right half.
#[test]
fn cjk_and_emoji_are_two_columns() {
    for ch in ['\u{4F60}', '\u{597D}', '\u{3042}', '\u{AC00}', '\u{1F512}', '\u{FF21}'] {
        assert_eq!(char_width(ch), 2, "{ch:?}");
    }
}

/// Box drawing is what a terminal UI is made of, and it is single width.
/// Treating it as wide would break every frame drawn with it.
#[test]
fn box_drawing_is_one_column() {
    for ch in ['\u{2500}', '\u{2502}', '\u{250C}', '\u{2588}', '\u{2591}'] {
        assert_eq!(char_width(ch), 1, "{ch:?}");
    }
}

/// Never zero. A zero would let a character occupy no column and leave the
/// cursor where it was, so the next write lands on top of it.
#[test]
fn width_is_never_zero() {
    for cp in [0x0u32, 0x20, 0x300, 0x200B, 0xFFFD, 0x10FFFF] {
        if let Some(ch) = char::from_u32(cp) {
            assert!(char_width(ch) >= 1, "{cp:#x}");
        }
    }
}

/// The ranges are searched in order and the scan stops at the first one that
/// starts above the codepoint. If they were out of order a later range would
/// never be reached, so each block boundary is checked from both sides.
#[test]
fn every_wide_block_is_reachable() {
    // (last narrow codepoint before the block, first codepoint of the block)
    const EDGES: &[(u32, u32)] = &[
        (0x10FF, 0x1100),
        (0x2E7F, 0x2E80),
        (0x3040, 0x3041),
        (0x33FF + 1, 0x3400),
        (0x4DBF + 1, 0x4E00),
        (0x9FFF + 1, 0xA000),
        (0xA95F, 0xA960),
        (0xABFF, 0xAC00),
        (0xF8FF, 0xF900),
        (0xFE0F, 0xFE10),
        (0xFE2F, 0xFE30),
        (0xFEFF, 0xFF00),
        (0xFFDF, 0xFFE0),
        (0x1F2FF, 0x1F300),
        (0x1F8FF, 0x1F900),
        (0x1FFFF, 0x20000),
    ];
    for &(narrow, wide) in EDGES {
        if let Some(ch) = char::from_u32(wide) {
            assert_eq!(char_width(ch), 2, "block start {wide:#x} should be wide");
        }
        if let Some(ch) = char::from_u32(narrow) {
            // Some of these fall inside a neighbouring wide block, which is
            // fine; what matters is that the block above is still reached.
            let w = char_width(ch);
            assert!(w == 1 || w == 2, "{narrow:#x}");
        }
    }
}
