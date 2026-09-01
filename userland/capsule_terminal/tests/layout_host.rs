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
            pub right: u32,
        }

        #[derive(Clone, Copy)]
        pub struct Layout {
            pub titlebar: Rect,
            pub tabstrip: Rect,
            pub left_rail: Rect,
            pub right_rail: Rect,
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
use layout::limits::{LEFT_RAIL_MIN_W, MIN_BODY_W, RIGHT_RAIL_MIN_W};
use layout::types::{Chrome, Layout, Rails};

const CHROME: Chrome = Chrome {
    titlebar_h: 28,
    tabstrip_h: 16,
    body_pad_top: 6,
    footer_h: 16,
    text_left: 14,
    row_h: 20,
};

const NO_RAILS: Rails = Rails { left: 0, right: 0 };
const BOTH_RAILS: Rails = Rails { left: 232, right: 250 };

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

/// The regression guard for the refactor that introduced this module: these
/// are the exact numbers the pre-refactor constants produced at the shipping
/// window size, so the default render must not have moved by a pixel.
#[test]
fn the_default_render_is_unchanged() {
    let l = lay(520, 300, NO_RAILS);
    assert_eq!(l.body.y, 50, "BODY_TOP was HEADER_H + 6 + 16");
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
    assert_eq!(l.right_rail.w, 0);
}

/// The three content columns must account for the whole width, or the
/// rightmost pixels of the window are never painted by anyone.
#[test]
fn the_rails_and_the_body_sum_to_the_width() {
    let l = lay(1440, 900, BOTH_RAILS);
    assert_eq!(l.left_rail.w + l.body.w + l.right_rail.w, 1440);
    assert_eq!(l.body.x, l.left_rail.w);
    assert_eq!(l.right_rail.x, l.left_rail.w + l.body.w);
}

/// Shrinking the window sheds the less important column first: the right
/// rail goes before the left one.
#[test]
fn the_right_rail_drops_first() {
    let l = lay(800, 900, BOTH_RAILS);
    assert_eq!(l.right_rail.w, 0);
    assert!(l.left_rail.w > 0, "the left rail still fits at 800");
}

#[test]
fn both_rails_drop_on_a_narrow_window() {
    let l = lay(520, 300, BOTH_RAILS);
    assert_eq!(l.left_rail.w, 0);
    assert_eq!(l.right_rail.w, 0);
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
            ("right_rail", l.right_rail),
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
    assert_eq!(RIGHT_RAIL_MIN_W, 900);
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
