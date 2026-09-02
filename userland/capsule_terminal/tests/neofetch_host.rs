// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
// Host-side harness for the two-column `neofetch` splash: the emblem's column
// width, the zip that lays it beside the info rows, and the colour strip.
// Compiled with the host toolchain:
//   rustc --edition 2021 --test tests/neofetch_host.rs -o /tmp/neofetch_host && /tmp/neofetch_host
//
// Only the pure modules are pulled in. `run.rs` is left out on purpose: it
// borrows `State` and calls `nonos_libc`, neither of which resolves outside
// cargo. The modules are included by path so this cannot pass against a copy
// that has drifted from what the capsule runs.

extern crate alloc;

#[path = "../src/command/builtin/neofetch"]
mod neofetch {
    pub mod compose;
    pub mod logo;
    pub mod palette;
}

use neofetch::compose::two_column;
use neofetch::logo::{LOGO, LOGO_W};
use neofetch::palette::palette;

fn text(row: &[u8]) -> String {
    String::from_utf8(row.to_vec()).expect("a composed row is utf8")
}

fn strip(row: &[u8]) -> String {
    let mut out = Vec::new();
    let mut rest = row;
    while let Some(at) = rest.iter().position(|b| *b == 0x1b) {
        out.extend_from_slice(&rest[..at]);
        let end = rest[at..].iter().position(|b| *b == b'm').expect("an sgr escape ends in m");
        rest = &rest[at + end + 1..];
    }
    out.extend_from_slice(rest);
    text(&out)
}

fn info(lines: &[&str]) -> Vec<Vec<u8>> {
    lines.iter().map(|l| l.as_bytes().to_vec()).collect()
}

/// The emblem is a grid, not a picture: one short row slides every info line
/// under it out of the column.
#[test]
fn every_logo_row_is_exactly_the_declared_width() {
    for (index, art) in LOGO.iter().enumerate() {
        assert_eq!(art.chars().count(), LOGO_W, "row {index}: {art:?}");
    }
}

/// The rows are box drawing glyphs, so the byte length is far past the column
/// count. Anything measuring with `len()` would indent the info column by
/// three times the intended amount.
#[test]
fn the_declared_width_is_columns_and_not_bytes() {
    assert!(LOGO.iter().any(|art| art.len() > LOGO_W));
    assert_eq!(LOGO[0].len(), LOGO_W);
}

#[test]
fn the_info_column_starts_at_the_same_offset_on_every_row() {
    let rows = two_column(&LOGO, &info(&["alpha", "beta", "gamma"]), 2);
    for (row, want) in rows.iter().zip(["alpha", "beta", "gamma"]) {
        let line = text(row);
        let at = line.char_indices().find(|(_, c)| *c == want.as_bytes()[0] as char);
        assert_eq!(line.chars().count() - want.chars().count(), LOGO_W + 2, "{line:?}");
        assert!(at.is_some());
        assert!(line.ends_with(want), "{line:?}");
    }
}

/// More info rows than emblem rows is the normal case: the strip has to keep
/// running with blank art on the left.
#[test]
fn a_longer_right_column_keeps_its_indent_past_the_emblem() {
    let right = info(&["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"]);
    let rows = two_column(&LOGO, &right, 2);
    assert_eq!(rows.len(), right.len());
    for row in &rows {
        assert_eq!(text(row).chars().count(), LOGO_W + 2 + 1);
    }
    assert!(!text(&rows[9]).contains('│'));
}

#[test]
fn a_longer_left_column_emits_the_art_alone() {
    let rows = two_column(&LOGO, &info(&["only"]), 2);
    assert_eq!(rows.len(), LOGO.len());
    assert_eq!(text(&rows[0]), format!("{}  only", LOGO[0]));
    assert_eq!(text(&rows[1]), LOGO[1].trim_end());
}

/// A blank emblem row against a blank info row must not leave a line of
/// invisible padding in the scrollback.
#[test]
fn trailing_padding_never_survives() {
    let rows = two_column(&LOGO, &[], 2);
    assert_eq!(text(&rows[0]), "");
    for row in &rows {
        assert!(!text(row).ends_with(' '), "{:?}", text(row));
    }
}

#[test]
fn two_empty_columns_compose_to_nothing() {
    assert!(two_column(&[], &[], 2).is_empty());
    assert!(two_column(&[], &[], 0).is_empty());
}

/// The whole point of the (plain, styled) pair: a redirected `neofetch` must
/// write glyphs to the file, never terminal control codes.
#[test]
fn the_plain_strip_carries_no_escape_at_all() {
    let (plain, _) = palette();
    assert!(!plain.contains(&0x1b));
    assert_eq!(text(&plain).chars().count(), 16);
}

#[test]
fn the_styled_strip_carries_one_escape_per_block() {
    let (plain, styled) = palette();
    assert_eq!(styled.iter().filter(|b| **b == 0x1b).count(), 8);
    assert_eq!(strip(&styled), text(&plain));
}

/// The strip is the point of reference for every other colour the terminal
/// prints, so the codes are the standard eight and stay in order.
#[test]
fn the_styled_strip_names_the_standard_colours_in_order() {
    let (_, styled) = palette();
    let line = text(&styled);
    for code in ["\x1b[31m", "\x1b[32m", "\x1b[33m", "\x1b[34m", "\x1b[35m", "\x1b[36m",
        "\x1b[37m", "\x1b[90m"]
    {
        assert!(line.contains(code), "{code:?}");
    }
    assert!(line.find("\x1b[31m") < line.find("\x1b[90m"));
}
