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
//
// Host-side harness for the command palette's matcher, query buffer and panel
// geometry. Compiled with the host toolchain:
//   rustc --edition 2021 --test tests/palette_host.rs -o /tmp/h && /tmp/h
//
// `Rect` comes from `nonos_toolkit`, which does not resolve outside cargo, so
// it is mirrored here as plain data. `index.rs` is left out on purpose: it
// borrows `State`, which drags in the whole capsule.

#[path = "../src/layout"]
mod layout {
    pub mod types {
        #[derive(Clone, Copy, PartialEq, Debug, Default)]
        pub struct Rect {
            pub x: u32,
            pub y: u32,
            pub w: u32,
            pub h: u32,
        }
    }

    pub use types::Rect;
}

#[path = "../src/palette"]
mod palette {
    pub mod entry;
    pub mod filter;
    pub mod geom;
    pub mod state;
    pub mod state_edit;
    pub mod verbs;
}

use layout::Rect;
use palette::entry::{Action, Entry, Kind};
use palette::filter::{filter, matches};
use palette::geom::{panel, query_row, row, rows_fit, MAX_ROWS};
use palette::state::{Palette, QUERY_CAP};
use palette::verbs::VERBS;

const BODY: Rect = Rect { x: 240, y: 60, w: 1200, h: 700 };

fn e(kind: Kind, label: &str) -> Entry {
    Entry { kind, label, hint: "", action: Action::Run }
}

fn index() -> Vec<Entry<'static>> {
    vec![
        e(Kind::Command, "clear"),
        e(Kind::Command, "history"),
        e(Kind::History, "git status"),
        e(Kind::History, "cargo check"),
        e(Kind::Session, "src"),
        e(Kind::Project, "/home/user/src"),
        e(Kind::Action, "Change Theme"),
    ]
}

#[test]
fn an_empty_query_keeps_the_whole_index_in_order() {
    let items = index();
    let mut out = [0usize; 16];
    let n = filter(&items, b"", &mut out);
    assert_eq!(n, items.len());
    assert_eq!(&out[..n], &[0, 1, 2, 3, 4, 5, 6]);
}

#[test]
fn a_query_that_matches_nothing_yields_nothing() {
    let mut out = [0usize; 16];
    assert_eq!(filter(&index(), b"zzzz", &mut out), 0);
}

#[test]
fn ranking_follows_source_order_then_recency() {
    let items = index();
    let mut out = [0usize; 16];
    let n = filter(&items, b"s", &mut out);
    let got: Vec<&str> = out[..n].iter().map(|&i| items[i].label).collect();
    assert_eq!(got, ["history", "git status", "src", "/home/user/src"]);
}

#[test]
fn matching_ignores_case_and_finds_the_middle_of_a_label() {
    assert!(matches("Change Theme", b"theme"));
    assert!(matches("cargo check", b"GO CH"));
    assert!(!matches("ls", b"lsx"));
    assert!(matches("", b""));
    assert!(!matches("", b"a"));
}

#[test]
fn a_query_longer_than_the_buffer_never_panics() {
    let mut p = Palette::new();
    p.show();
    for i in 0..QUERY_CAP * 4 {
        p.push(b'a' + (i % 26) as u8);
    }
    assert_eq!(p.qlen, QUERY_CAP);
    assert_eq!(p.needle().len(), QUERY_CAP);
    let mut out = [0usize; 16];
    assert_eq!(filter(&index(), p.needle(), &mut out), 0);
    for _ in 0..QUERY_CAP * 4 {
        p.backspace();
    }
    assert_eq!(p.qlen, 0);
    assert_eq!(filter(&index(), p.needle(), &mut out), index().len());
}

#[test]
fn a_label_shorter_than_the_query_is_not_a_match() {
    let long = [b'a'; QUERY_CAP];
    assert!(!matches("a", &long));
}

#[test]
fn selection_wraps_and_survives_an_empty_result_set() {
    let mut p = Palette::new();
    p.step(-1, 0);
    assert_eq!(p.sel, 0);
    p.step(-1, 4);
    assert_eq!(p.sel, 3);
    p.step(1, 4);
    assert_eq!(p.sel, 0);
    p.push(b'x');
    assert_eq!(p.sel, 0);
}

#[test]
fn hiding_clears_the_query_so_the_next_open_starts_empty() {
    let mut p = Palette::new();
    p.show();
    p.push(b'g');
    p.step(1, 3);
    p.hide();
    assert!(!p.open);
    assert_eq!(p.needle(), b"");
    assert_eq!(p.sel, 0);
}

#[test]
fn the_panel_and_its_rows_stay_inside_the_body() {
    for rows in 0..=MAX_ROWS as u32 {
        let p = panel(BODY, 30, rows);
        assert!(p.x >= BODY.x && p.x + p.w <= BODY.x + BODY.w, "rows {rows} width");
        assert!(p.y >= BODY.y && p.y + p.h <= BODY.y + BODY.h, "rows {rows} height");
        for i in 0..rows_fit(p, 30) {
            let r = row(p, i, 30);
            assert!(r.y + r.h <= p.y + p.h, "rows {rows} row {i} overruns");
            assert!(r.x >= p.x && r.x + r.w <= p.x + p.w);
        }
    }
}

#[test]
fn rows_never_overlap_the_query_row() {
    let p = panel(BODY, 30, 6);
    let q = query_row(p, 30);
    assert!(row(p, 0, 30).y >= q.y + q.h);
    for i in 1..rows_fit(p, 30) {
        let prev = row(p, i - 1, 30);
        assert_eq!(prev.y + prev.h, row(p, i, 30).y, "row {i}");
    }
}

#[test]
fn a_body_too_small_for_the_panel_degrades_instead_of_wrapping() {
    for h in 0..120u32 {
        let body = Rect { x: 0, y: 0, w: 200, h };
        let p = panel(body, 30, 8);
        assert!(p.y + p.h <= body.y + body.h, "h {h}");
        assert!(p.x + p.w <= body.w, "h {h} width");
        assert!(rows_fit(p, 30) * 30 <= p.h);
    }
}

#[test]
fn every_verb_is_a_bare_word_the_dispatcher_can_run() {
    for (verb, hint) in VERBS.iter() {
        assert!(!verb.is_empty());
        assert!(!verb.contains(' '), "{verb}");
        assert!(!hint.is_empty(), "{verb}");
        assert!(matches(verb, verb.as_bytes()));
    }
}
