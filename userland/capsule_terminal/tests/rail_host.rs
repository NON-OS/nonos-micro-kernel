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
// Host-side harness for the rail's navigation rows and their hit-test.
// Compiled with the host toolchain:
//   rustc --edition 2021 --test tests/rail_host.rs -o /tmp/rail_host && /tmp/rail_host
//
// `Rect` comes from `nonos_toolkit` and `lh()` from the TTF metrics, neither of
// which resolves outside cargo, so both are mirrored here as plain data. The
// arithmetic under test lives in the real `rail_band.rs`, `rail_row.rs` and
// `rail_left_geom.rs`, which are pulled in by path. The scrolled-column
// geometry is covered by `rail_scroll_host.rs`, the formatters by
// `rail_fmt_host.rs`.

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

#[path = "../src/term"]
mod term {
    pub mod util {
        pub mod copy_into;
        pub mod format_u64;

        pub use copy_into::copy_into;
        pub use format_u64::format_u64;
    }
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

    pub mod rail_fmt;

    pub mod rail_metric;

    pub mod rail_addr;
}

#[path = "../src/rail"]
mod rail {
    pub mod disk;
    pub mod mem;
    pub mod metrics;
    pub mod net;
    pub mod net_decode;
    pub mod value;
}

use layout::Rect;
use paint::rail_addr::{ipv4_pfx, ipv6_str};
use paint::rail_band::Band;
use paint::rail_left_geom::{hit, nav_sections, LeftHit};
use paint::rail_metric::DASH;
use paint::rail_row::{base_name, row_at, row_band, row_h};
use rail::mem::summarize;
use rail::metrics::{Proc, Sample};
use rail::net::Net;
use rail::net_decode::{decode_lease, HDR_LEN, REPLY_LEN};
use rail::value::Metric;

const RAIL: Rect = Rect { x: 0, y: 40, w: 240, h: 900 };

fn list(offset: u32) -> Band {
    nav_sections(RAIL, offset, 4, 3).s_list
}

#[test]
fn rows_tile_without_overlap_or_gap() {
    let l = Band { x: 12, y: 100, w: 216, h: 9 * row_h() };
    for i in 0..9 {
        let a = row_band(i, &l);
        assert_eq!(a.h, row_h(), "row {i} height");
        assert_eq!(a.x, l.x);
        assert_eq!(a.w, l.w);
        assert_eq!(a.y, l.y + (i * row_h()) as i32, "row {i} top");
        if i > 0 {
            let prev = row_band(i - 1, &l);
            assert_eq!(prev.y + prev.h as i32, a.y, "row {i} abuts row {}", i - 1);
        }
    }
}

#[test]
fn a_scrolled_row_keeps_its_place_in_the_column() {
    let rest = row_band(2, &list(0));
    let moved = row_band(2, &list(70));
    assert_eq!(moved.y, rest.y - 70, "the column translates, it does not relayout");
    assert_eq!(moved.h, rest.h);
    assert!(row_band(0, &list(1000)).y < 0, "a row above the viewport has a negative top");
}

#[test]
fn hit_test_agrees_with_the_painter_at_every_index_and_offset() {
    for off in [0u32, 17, 60, 200] {
        let l = list(off);
        for i in 0..4 {
            let r = row_band(i, &l);
            for (x, y) in [
                (r.x, r.y),
                (r.x + r.w - 1, r.y),
                (r.x, r.y + r.h as i32 - 1),
                (r.x + r.w / 2, r.y + r.h as i32 / 2),
            ] {
                assert_eq!(row_at(&l, 4, x, y), Some(i), "off {off} row {i} at ({x},{y})");
            }
        }
    }
}

#[test]
fn a_point_outside_the_list_hits_no_row() {
    let l = list(0);
    assert_eq!(row_at(&l, 9, l.x, l.y - 1), None);
    assert_eq!(row_at(&l, 9, l.x + l.w, l.y), None);
    assert_eq!(row_at(&l, 9, l.x, l.y + (9 * row_h()) as i32), None);
}

#[test]
fn a_row_past_the_count_is_not_clickable() {
    let l = list(0);
    let r = row_band(2, &l);
    assert_eq!(row_at(&l, 2, r.x + 1, r.y + 1), None);
    assert_eq!(row_at(&l, 3, r.x + 1, r.y + 1), Some(2));
}

#[test]
fn hit_routes_captions_before_rows_and_follows_the_offset() {
    for off in [0u32, 40] {
        let g = nav_sections(RAIL, off, 4, 2);
        let plus = g.s_plus;
        let (px, py) = (RAIL.x + plus.x + plus.w / 2, (RAIL.y as i32 + plus.y + 1) as u32);
        assert!(matches!(hit(RAIL, 4, 2, off, px, py), Some(LeftHit::NewSession)), "off {off}");
        let a = row_band(1, &g.s_list);
        let ay = (RAIL.y as i32 + a.y + 4) as u32;
        assert!(matches!(hit(RAIL, 4, 2, off, RAIL.x + a.x + 4, ay), Some(LeftHit::Session(1))));
        let b = row_band(1, &g.p_list);
        let by = (RAIL.y as i32 + b.y + 4) as u32;
        assert!(matches!(hit(RAIL, 4, 2, off, RAIL.x + b.x + 4, by), Some(LeftHit::Project(1))));
    }
}

#[test]
fn the_lease_the_rail_shows_is_the_lease_the_decoder_read() {
    let mut b = [0u8; 64];
    let n = decode_lease(&lease(3, [10, 0, 2, 15], 24));
    assert_eq!(ipv4_pfx(&mut b, n.ipv4, n.prefix_len), "10.0.2.15/24");
    assert_eq!(ipv6_str(&mut b, n.ipv6), DASH, "no v6 stack, so no v6 row figure");
    assert_eq!(ipv6_str(&mut b, Metric::Known([0; 16])), "0000:0000:0000:0000:0000:0000:0000:0000");
}

#[test]
fn base_name_takes_the_last_component() {
    assert_eq!(base_name("/home/user/src"), "src");
    assert_eq!(base_name("/"), "/");
    assert_eq!(base_name(""), "");
    assert_eq!(base_name("bare"), "bare");
    assert_eq!(base_name("/trailing/"), "/trailing/");
}

fn proc_with(pid: u32, mem_kb: u64) -> Proc {
    Proc { pid, mem_kb, ..Proc::EMPTY }
}

fn lease(state: u8, ip: [u8; 4], prefix: u8) -> [u8; REPLY_LEN] {
    let mut rx = [0u8; REPLY_LEN];
    rx[HDR_LEN] = state;
    rx[HDR_LEN + 1..HDR_LEN + 5].copy_from_slice(&ip);
    rx[HDR_LEN + 5] = prefix;
    rx[HDR_LEN + 6..HDR_LEN + 10].copy_from_slice(&[10, 0, 2, 2]);
    rx
}

#[test]
fn memory_used_is_the_resident_sum_and_saturates() {
    let live = [proc_with(1, 4096), proc_with(2, 512), proc_with(3, 0)];
    assert_eq!(summarize(&live).used_kb, Metric::Known(4608));
    let huge = [proc_with(1, u64::MAX), proc_with(2, 1024)];
    assert_eq!(summarize(&huge).used_kb, Metric::Known(u64::MAX));
}

#[test]
fn an_empty_process_table_is_unknown_memory_rather_than_zero() {
    let m = summarize(&[]);
    assert_eq!(m.used_kb, Metric::Unknown);
    assert!(!m.used_kb.is_known() && !m.used_kb.is_unsupported());
}

#[test]
fn memory_total_and_swap_have_no_source_at_all() {
    let m = summarize(&[proc_with(1, 8)]);
    assert!(m.total_kb.is_unsupported() && m.swap_used_kb.is_unsupported());
}

#[test]
fn a_bound_lease_yields_the_real_address() {
    let n = decode_lease(&lease(3, [10, 0, 2, 15], 24));
    assert!(n.up && n.name_str() == "net0");
    assert_eq!(n.ipv4, Metric::Known([10, 0, 2, 15]));
    assert_eq!(n.prefix_len, Metric::Known(24));
    assert_eq!(n.gateway, Metric::Known([10, 0, 2, 2]));
}

#[test]
fn a_pre_bound_or_short_reply_leaves_the_interface_down() {
    assert_eq!(decode_lease(&lease(2, [10, 0, 2, 15], 24)), Net::DOWN);
    assert_eq!(decode_lease(&[0u8; 8]), Net::DOWN);
    assert!(!Net::DOWN.up && Net::DOWN.ipv4 == Metric::Unknown);
}

#[test]
fn the_figures_nonos_cannot_measure_stay_unsupported() {
    let s = Sample::EMPTY;
    let up = decode_lease(&lease(3, [1, 2, 3, 4], 8));
    assert!(up.ipv6.is_unsupported() && up.rx_bps.is_unsupported() && up.tx_bps.is_unsupported());
    assert!(s.disk.total_kb.is_unsupported() && s.disk.used_kb.is_unsupported());
    assert!(s.load_avg.is_unsupported());
    assert_eq!(Metric::Known(7u32).value(), Some(7));
}
