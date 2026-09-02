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
// Host-side harness for the window band/rail solver. Compiled with the host
// toolchain:
//   rustc --edition 2021 --test tests/layout_host.rs -o /tmp/layout_host && /tmp/layout_host
//
// `src/layout/types.rs` re-exports `Rect` from `nonos_toolkit`, which does not
// resolve outside cargo, so `types` is mirrored here as plain data. The
// arithmetic under test lives entirely in the real `compute.rs` and
// `limits.rs`, which are pulled in by path.

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

        #[derive(Clone, Copy)]
        pub struct Chrome {
            pub titlebar_h: u32,
            pub tabstrip_h: u32,
            pub body_pad_top: u32,
            pub footer_h: u32,
            pub text_left: u32,
            pub row_h: u32,
        }

        #[derive(Clone, Copy)]
        pub struct Rails {
            pub left: u32,
        }

        #[derive(Clone, Copy)]
        pub struct Layout {
            pub titlebar: Rect,
            pub tabstrip: Rect,
            pub left_rail: Rect,
            pub body: Rect,
            pub input: Rect,
            pub footer: Rect,
        }
    }

    pub mod limits;

    pub mod compute;
}

#[path = "../src/layout/rows.rs"]
mod rows;

use layout::compute::compute;
use layout::limits::{LEFT_RAIL_MIN_W, LEFT_RAIL_W, MIN_BODY_W};
use layout::types::{Chrome, Layout, Rails};

const CHROME: Chrome = Chrome {
    titlebar_h: 28,
    tabstrip_h: 0,
    body_pad_top: 6,
    footer_h: 16,
    text_left: 14,
    row_h: 20,
};

const NO_RAILS: Rails = Rails { left: 0 };
const BOTH_RAILS: Rails = Rails { left: LEFT_RAIL_W };

fn lay(w: u32, h: u32, r: Rails) -> Layout {
    compute(w, h, &CHROME, r)
}

/// Every horizontal band has to abut the next one exactly: a gap paints
/// wallpaper through the chrome, an overlap paints one band over another.
#[test]
fn the_bands_tile_the_window_with_no_gap_or_overlap() {
    let l = lay(1440, 900, NO_RAILS);
    assert_eq!(l.titlebar.y, 0);
    assert_eq!(l.tabstrip.y, l.titlebar.y + l.titlebar.h);
    assert_eq!(l.body.y, l.tabstrip.y + l.tabstrip.h + CHROME.body_pad_top);
    assert_eq!(l.input.y, l.body.y + l.body.h);
    assert_eq!(l.footer.y, l.input.y + l.input.h);
    assert_eq!(l.footer.y + l.footer.h, 900);
}

/// The regression guard for the shipping window geometry. `body.y` was 50 while
/// the content area still reserved 16px for the in-content tab strip; the strip
/// moved into the titlebar accessory, that band was reclaimed, and the body top
/// is now HEADER_H + BODY_PAD_TOP = 34. The guard is not weakened: `input.y` and
/// `footer.y` are anchored to the bottom of the window and must not have moved.
#[test]
fn the_default_render_pins_the_reclaimed_geometry() {
    let l = lay(520, 300, NO_RAILS);
    assert_eq!(l.body.y, 34, "BODY_TOP is HEADER_H + BODY_PAD_TOP");
    assert_eq!(l.input.y, 264, "input_y was 300 - (FOOTER_H + row_h)");
    assert_eq!(l.footer.y, 284, "body_max was 300 - FOOTER_H");
    assert_eq!(l.body.x + CHROME.text_left, 14, "TEXT_LEFT with no left rail");
}

#[test]
fn the_body_spans_the_full_width_without_rails() {
    let l = lay(1440, 900, NO_RAILS);
    assert_eq!(l.body.x, 0);
    assert_eq!(l.body.w, 1440);
    assert_eq!(l.left_rail.w, 0);
}

/// The two content columns must account for the whole width, or the
/// rightmost pixels of the window are never painted by anyone.
#[test]
fn the_rails_and_the_body_sum_to_the_width() {
    let l = lay(1440, 900, BOTH_RAILS);
    assert_eq!(l.left_rail.w + l.body.w, 1440);
    assert_eq!(l.body.x, l.left_rail.w);
    assert_eq!(l.body.x + l.body.w, 1440);
}

/// The rail hosts a four-column process table drawn at ttf::MIN_UI_PX, so its
/// width is a hard requirement, not a taste: PID 40 + CPU 46 + MEM 46 + one
/// 8px gutter + 12px padding either side leaves 156px for the capsule name.
#[test]
fn the_left_rail_fits_the_process_table() {
    const PID_W: u32 = 40;
    const CPU_W: u32 = 46;
    const MEM_W: u32 = 46;
    const RAIL_GAP: u32 = 8;
    const RAIL_PAD: u32 = 12;
    let content = LEFT_RAIL_W - RAIL_PAD * 2;
    let name_w = content - (PID_W + CPU_W + MEM_W + RAIL_GAP);
    assert!(name_w >= 150, "name column starved at {}", name_w);
    let l = lay(1280, 720, BOTH_RAILS);
    assert_eq!(l.left_rail.w, LEFT_RAIL_W, "the rail is not clipped at 1280");
    assert_eq!(l.left_rail.x, 0);
}

/// The rail and the body may never share a pixel column: the body starts where
/// the rail ends, at every width the rail survives.
#[test]
fn the_body_never_overlaps_the_left_rail() {
    for w in [640u32, 700, 900, 1024, 1280, 1440, 2560] {
        let l = lay(w, 720, BOTH_RAILS);
        assert_eq!(l.body.x, l.left_rail.x + l.left_rail.w, "overlap at {}", w);
        assert_eq!(l.input.x, l.body.x, "input drifted from the body at {}", w);
        assert_eq!(l.left_rail.w + l.body.w, w, "width unaccounted at {}", w);
    }
}

/// The body has to stay a usable terminal at the shipping 1280x720 guest.
#[test]
fn the_body_stays_wide_at_the_guest_resolution() {
    let l = lay(1280, 720, BOTH_RAILS);
    assert_eq!(l.body.w, 1280 - LEFT_RAIL_W);
    assert!(l.body.w >= 900, "body squeezed to {}", l.body.w);
}

#[test]
fn the_rail_drops_on_a_narrow_window() {
    let l = lay(520, 300, BOTH_RAILS);
    assert_eq!(l.left_rail.w, 0);
    assert_eq!(l.body.w, 520);
}

/// Whatever the rails ask for, the terminal itself must keep a usable column;
/// a body squeezed to nothing is a window with no output in it.
#[test]
fn the_body_never_starves() {
    for w in [320u32, 520, 640, 700, 900, 1024, 1440, 2560] {
        let l = lay(w, 900, BOTH_RAILS);
        assert!(
            l.body.w >= MIN_BODY_W.min(w),
            "body starved at {}: got {}",
            w,
            l.body.w
        );
    }
}

/// The real payoff of a host harness: these sizes arise from a drag or a
/// first frame before the compositor reports a size, and never appear in a
/// screendump. Any subtraction that underflows here panics in debug and wraps
/// to a multi-gigabyte rect in release.
#[test]
fn degenerate_windows_do_not_underflow() {
    for (w, h) in [(0u32, 0u32), (1, 1), (10, 10), (100, 40), (320, 60)] {
        let l = lay(w, h, BOTH_RAILS);
        for (name, r) in [
            ("titlebar", l.titlebar),
            ("tabstrip", l.tabstrip),
            ("left_rail", l.left_rail),
            ("body", l.body),
            ("input", l.input),
            ("footer", l.footer),
        ] {
            assert!(r.w <= w, "{} wider than {}x{}: {:?}", name, w, h, r);
            assert!(r.h <= h, "{} taller than {}x{}: {:?}", name, w, h, r);
        }
    }
}

/// The prompt sits directly under the scrollback, so it has to share the
/// body's column exactly or the caret drifts away from the text above it.
#[test]
fn the_input_shares_the_body_column() {
    let l = lay(1440, 900, BOTH_RAILS);
    assert_eq!(l.input.x, l.body.x);
    assert_eq!(l.input.w, l.body.w);
    assert_eq!(l.input.h, CHROME.row_h);
}

/// The breakpoints are part of the layout spec, not incidental numbers; a
/// silent edit here changes which chrome appears at which window size.
#[test]
fn the_thresholds_match_the_spec() {
    assert_eq!(MIN_BODY_W, 320);
    assert_eq!(LEFT_RAIL_MIN_W, 640);
    assert_eq!(LEFT_RAIL_W, 320);
    assert_eq!(LEFT_RAIL_MIN_W, LEFT_RAIL_W + MIN_BODY_W);
}

/// The regression guard for the block-chrome drift bug: one painter stepped
/// rows by a constant 15 while another stepped by the measured line height,
/// so the two agreed only at font_scale 2 and separated everywhere else.
#[test]
fn row_top_advances_by_the_measured_line_height() {
    for lh in [13u32, 15, 17, 20, 23, 27] {
        for i in 0u32..15 {
            assert_eq!(
                rows::row_top(i, 50, lh),
                50 + i * lh,
                "row {} at line height {}",
                i,
                lh
            );
        }
    }
}

/// The rail is one scrolling column, so a section stack taller than the rail
/// must scroll exactly its overflow, and a short stack must not scroll at all.
#[test]
fn the_rail_scroll_extent_is_the_overflow_only() {
    assert_eq!(rows::scroll_max(900, 600), 300);
    assert_eq!(rows::scroll_max(400, 600), 0);
    assert_eq!(rows::scroll_clamp(999, 900, 600), 300);
    assert_eq!(rows::scroll_clamp(120, 900, 600), 120);
    assert_eq!(rows::scroll_clamp(5, 400, 600), 0);
}
