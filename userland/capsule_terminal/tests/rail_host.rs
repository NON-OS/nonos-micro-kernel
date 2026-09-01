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
// Host-side harness for the left rail's row geometry. Compiled with the host
// toolchain:
//   rustc --edition 2021 --test tests/rail_host.rs -o /tmp/rail_host && /tmp/rail_host
//
// `Rect` comes from `nonos_toolkit` and `lh()` from the TTF metrics, neither of
// which resolves outside cargo, so both are mirrored here as plain data. The
// arithmetic under test lives in the real `rail_row.rs` and `rail_left_geom.rs`,
// which are pulled in by path.

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

#[path = "../src/paint"]
mod paint {
    pub mod rail_text {
        pub const RAIL_PAD: u32 = 12;
        pub const RAIL_GAP: u32 = 8;

        pub fn lh() -> u32 {
            22
        }
    }

    pub mod rail_row;

    pub mod rail_left_geom;
}

use layout::Rect;
use paint::rail_left_geom::{hit, sections, LeftHit};
use paint::rail_row::{base_name, inside, row_at, row_h, row_rect, rows_fit};

const RAIL: Rect = Rect { x: 0, y: 40, w: 240, h: 900 };

fn list() -> Rect {
    sections(RAIL, 4).s_list
}

#[test]
fn rows_tile_without_overlap_or_gap() {
    let l = Rect { x: 12, y: 100, w: 216, h: 600 };
    for i in 0..rows_fit(l) {
        let a = row_rect(i, l);
        assert_eq!(a.h, row_h(), "row {i} height");
        assert_eq!(a.x, l.x);
        assert_eq!(a.w, l.w);
        assert_eq!(a.y, l.y + i * row_h(), "row {i} top");
        if i > 0 {
            let prev = row_rect(i - 1, l);
            assert_eq!(prev.y + prev.h, a.y, "row {i} abuts row {}", i - 1);
        }
    }
}

#[test]
fn rows_fit_never_overruns_the_list() {
    for h in 0..400u32 {
        let l = Rect { x: 0, y: 0, w: 100, h };
        let n = rows_fit(l);
        assert!(n * row_h() <= h, "h {h} fits {n}");
        assert!((n + 1) * row_h() > h, "h {h} could fit {}", n + 1);
    }
}

#[test]
fn hit_test_agrees_with_the_painter_at_every_index() {
    let l = list();
    for i in 0..rows_fit(l).min(9) {
        let r = row_rect(i, l);
        for (x, y) in [
            (r.x, r.y),
            (r.x + r.w - 1, r.y),
            (r.x, r.y + r.h - 1),
            (r.x + r.w - 1, r.y + r.h - 1),
            (r.x + r.w / 2, r.y + r.h / 2),
        ] {
            assert_eq!(row_at(l, 9, x, y), Some(i), "row {i} corner ({x},{y})");
        }
    }
}

#[test]
fn a_point_outside_the_list_hits_no_row() {
    let l = list();
    assert_eq!(row_at(l, 9, l.x, l.y.saturating_sub(1)), None);
    assert_eq!(row_at(l, 9, l.x + l.w, l.y), None);
    assert_eq!(row_at(l, 9, l.x, l.y + rows_fit(l) * row_h()), None);
}

#[test]
fn a_row_past_the_count_is_not_clickable() {
    let l = list();
    let r = row_rect(2, l);
    assert_eq!(row_at(l, 2, r.x + 1, r.y + 1), None);
    assert_eq!(row_at(l, 3, r.x + 1, r.y + 1), Some(2));
}

#[test]
fn sections_stack_inside_the_rail_and_never_overlap() {
    for n in 0..12u32 {
        let s = sections(RAIL, n);
        assert!(s.s_head.y >= RAIL.y);
        assert_eq!(s.s_list.y, s.s_head.y + s.s_head.h);
        assert!(s.p_head.y >= s.s_list.y + s.s_list.h, "n {n} projects caption");
        assert_eq!(s.p_list.y, s.p_head.y + s.p_head.h);
        assert!(s.p_list.y + s.p_list.h <= RAIL.y + RAIL.h, "n {n} overruns the rail");
        assert!(s.s_list.h <= RAIL.h, "n {n} sessions list");
    }
}

#[test]
fn a_short_rail_degrades_instead_of_wrapping() {
    for h in 0..200u32 {
        let s = sections(Rect { x: 0, y: 0, w: 240, h }, 9);
        assert!(s.s_list.h <= h);
        assert!(s.p_list.h <= h);
        assert_eq!(rows_fit(s.s_list) * row_h() <= s.s_list.h, true);
    }
}

#[test]
fn the_plus_affordances_sit_in_their_captions() {
    let s = sections(RAIL, 4);
    assert!(inside(s.s_plus, s.s_plus.x, s.s_plus.y));
    assert!(s.s_plus.x + s.s_plus.w <= s.s_head.x + s.s_head.w);
    assert!(s.p_plus.x + s.p_plus.w <= s.p_head.x + s.p_head.w);
    assert!(s.s_plus.y >= s.s_head.y && s.s_plus.y + s.s_plus.h <= s.s_head.y + s.s_head.h);
}

#[test]
fn hit_routes_captions_before_rows() {
    let s = sections(RAIL, 4);
    let plus = s.s_plus;
    assert!(matches!(
        hit(RAIL, 4, 2, plus.x + plus.w / 2, plus.y + plus.h / 2),
        Some(LeftHit::NewSession)
    ));
    let padd = s.p_plus;
    assert!(matches!(
        hit(RAIL, 4, 2, padd.x + padd.w / 2, padd.y + padd.h / 2),
        Some(LeftHit::AddProject)
    ));
}

#[test]
fn hit_separates_the_two_lists() {
    let s = sections(RAIL, 4);
    let a = row_rect(1, s.s_list);
    assert!(matches!(hit(RAIL, 4, 3, a.x + 4, a.y + 4), Some(LeftHit::Session(1))));
    let b = row_rect(2, s.p_list);
    assert!(matches!(hit(RAIL, 4, 3, b.x + 4, b.y + 4), Some(LeftHit::Project(2))));
}

#[test]
fn base_name_takes_the_last_component() {
    assert_eq!(base_name("/home/user/src"), "src");
    assert_eq!(base_name("/"), "/");
    assert_eq!(base_name(""), "");
    assert_eq!(base_name("bare"), "bare");
    assert_eq!(base_name("/trailing/"), "/trailing/");
}
