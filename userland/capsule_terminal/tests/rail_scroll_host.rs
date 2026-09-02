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
// Host-side harness for the rail's scrolled-column geometry: band clipping and
// visibility, the section heights and how the offset is clamped against the
// content. Compiled with the host toolchain:
//   rustc --edition 2021 --test tests/rail_scroll_host.rs -o /tmp/rail_scroll_host && /tmp/rail_scroll_host
//
// Split out of `rail_host.rs`, which covers the navigation rows and hit-test.
// `Rect` and `lh()` are mirrored here as plain data for the same reason.

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

    pub mod rows;

    pub use types::Rect;
}

#[path = "../src/paint"]
mod paint {
    pub mod rail_text {
        pub const RAIL_PAD: u32 = 12;
        pub const RAIL_GAP: u32 = 8;
        pub const BAR_H: u32 = 3;

        pub fn lh() -> u32 {
            22
        }
    }

    pub mod rail_band;

    pub mod rail_row;

    pub mod rail_left_geom;

    pub mod rail_geom;

    pub mod rail_scroll;
}

use layout::Rect;
use paint::rail_band::{clip, hits, visible, Band};
use paint::rail_geom::{disk_h, net_h, procs_h, sys_h, telemetry_h};
use paint::rail_left_geom::{nav_h, nav_sections, sections};
use paint::rail_row::{inside, row_h};
use paint::rail_scroll::{clamp, content_h, telemetry_top, RailFit};
use paint::rail_text::{lh, RAIL_GAP, RAIL_PAD};

const RAIL: Rect = Rect { x: 0, y: 40, w: 240, h: 900 };

fn fit(sessions: u32, projects: u32, procs: u32) -> RailFit {
    RailFit { sessions, projects, telemetry: true, procs }
}
#[test]
fn a_band_is_cut_at_the_top_and_dropped_once_it_is_wholly_above() {
    assert_eq!(clip(10, 20), Some((10, 20)));
    assert_eq!(clip(0, 20), Some((0, 20)));
    assert_eq!(clip(-5, 20), Some((0, 15)));
    assert_eq!(clip(-20, 20), None);
    assert_eq!(clip(-21, 20), None);
    assert_eq!(clip(-1, 1), None);
}

#[test]
fn visibility_covers_exactly_the_bands_that_touch_the_rail() {
    let b = |y| Band { x: 0, y, w: 10, h: 20 };
    assert!(!visible(&b(-20), 100));
    assert!(visible(&b(-19), 100));
    assert!(visible(&b(99), 100));
    assert!(!visible(&b(100), 100));
    assert!(inside(RAIL, RAIL.x, RAIL.y));
    assert!(!inside(RAIL, RAIL.x, RAIL.y - 1));
}

#[test]
fn the_section_heights_compose_into_the_telemetry_block() {
    assert_eq!(sys_h(), 34 + lh() * 5 + 3 + RAIL_GAP * 2 + 44);
    assert_eq!(net_h(), 34 + lh() * 4);
    assert_eq!(disk_h(), 34 + lh() * 2);
    assert_eq!(procs_h(0), 34 + lh() + RAIL_GAP / 2);
    assert_eq!(procs_h(7), procs_h(0) + 7 * lh());
    let want = sys_h() + net_h() + disk_h() + procs_h(5) + RAIL_GAP * 3 + RAIL_PAD;
    assert_eq!(telemetry_h(5), want, "the composer steps by exactly these heights");
}

#[test]
fn content_height_is_the_navigation_lists_plus_the_telemetry_block() {
    let f = fit(4, 3, 6);
    assert_eq!(content_h(f), nav_h(4, 3) + RAIL_GAP + telemetry_h(6));
    let off = RailFit { telemetry: false, ..f };
    assert_eq!(content_h(off), nav_h(4, 3), "no telemetry, no telemetry height");
    assert_eq!(telemetry_top(0, f), (nav_h(4, 3) + RAIL_GAP) as i32);
    assert_eq!(telemetry_top(90, f), (nav_h(4, 3) + RAIL_GAP) as i32 - 90);
}

#[test]
fn the_offset_is_clamped_at_both_ends() {
    let f = fit(4, 3, 40);
    let content = content_h(f);
    assert!(content > RAIL.h, "the fixture must actually overflow the rail");
    assert_eq!(clamp(0, f, RAIL.h), 0);
    assert_eq!(clamp(u32::MAX, f, RAIL.h), content - RAIL.h);
    assert_eq!(clamp(content - RAIL.h + 1, f, RAIL.h), content - RAIL.h);
    let short = fit(1, 0, 0);
    assert_eq!(clamp(500, short, 4000), 0, "content shorter than the rail never scrolls");
}

#[test]
fn sections_stack_and_nav_height_matches_what_they_consume() {
    for s in 0..12u32 {
        for p in 0..5u32 {
            let g = sections(RAIL_PAD, 216, 0, s, p);
            assert_eq!(g.s_list.y, g.s_head.y + g.s_head.h as i32);
            assert_eq!(g.p_head.y, g.s_list.y + (g.s_list.h + RAIL_GAP) as i32);
            assert_eq!(g.p_list.y, g.p_head.y + g.p_head.h as i32);
            assert_eq!(g.s_list.h, s * row_h());
            assert_eq!(g.p_list.h, p * row_h());
            let bottom = g.p_list.y + g.p_list.h as i32;
            assert_eq!(bottom, nav_h(s, p) as i32, "nav_h({s},{p}) tracks the last band");
        }
    }
}

#[test]
fn the_plus_affordances_sit_in_their_captions() {
    let g = nav_sections(RAIL, 0, 4, 3);
    assert!(hits(&g.s_plus, g.s_plus.x, g.s_plus.y));
    assert!(g.s_plus.x + g.s_plus.w <= g.s_head.x + g.s_head.w);
    assert!(g.p_plus.x + g.p_plus.w <= g.p_head.x + g.p_head.w);
    assert!(g.s_plus.y >= g.s_head.y && g.s_plus.y + g.s_plus.h as i32 <= g.s_head.y + g.s_head.h as i32);
}
