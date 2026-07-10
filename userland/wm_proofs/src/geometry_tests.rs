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

//! Hit-testing and clamping are what make click-to-raise and window placement
//! correct. `contains` decides which window a click lands on; `overlaps`
//! decides collision; `clamp_to_display` keeps a window on screen. These pin
//! the exact edge behavior, since an off-by-one at a window boundary is a click
//! that raises the wrong window.

use crate::constrain::{clamp_to_display, MIN_WINDOW_DIM};
use crate::rect::Rect;

fn r(x: u32, y: u32, w: u32, h: u32) -> Rect {
    Rect { x, y, width: w, height: h }
}

#[test]
fn contains_is_half_open_on_both_axes() {
    let win = r(10, 20, 100, 50);
    assert!(win.contains(10, 20), "top-left corner is inside");
    assert!(win.contains(109, 69), "bottom-right interior is inside");
    assert!(!win.contains(110, 20), "right edge is exclusive");
    assert!(!win.contains(10, 70), "bottom edge is exclusive");
    assert!(!win.contains(9, 20), "just left is outside");
}

#[test]
fn overlap_is_symmetric_and_edge_exclusive() {
    let a = r(0, 0, 20, 20);
    let b = r(20, 0, 20, 20); // shares the x=20 edge, does not overlap
    assert!(!a.overlaps(&b));
    assert!(!b.overlaps(&a));
    let c = r(19, 0, 20, 20); // one column of overlap
    assert!(a.overlaps(&c));
    assert!(c.overlaps(&a));
}

#[test]
fn contained_window_overlaps_its_container() {
    let outer = r(0, 0, 100, 100);
    let inner = r(40, 40, 10, 10);
    assert!(outer.overlaps(&inner));
    assert!(inner.overlaps(&outer));
}

#[test]
fn clamp_pulls_an_offscreen_window_fully_on_screen() {
    let clamped = clamp_to_display(r(2000, 2000, 400, 300), 1280, 800);
    assert!(clamped.x + clamped.width <= 1280, "right edge on screen");
    assert!(clamped.y + clamped.height <= 800, "bottom edge on screen");
    assert_eq!(clamped.width, 400);
    assert_eq!(clamped.height, 300);
}

#[test]
fn clamp_shrinks_a_window_larger_than_the_display() {
    let clamped = clamp_to_display(r(0, 0, 5000, 5000), 1280, 800);
    assert!(clamped.width <= 1280);
    assert!(clamped.height <= 800);
    assert!(clamped.x + clamped.width <= 1280);
    assert!(clamped.y + clamped.height <= 800);
}

#[test]
fn clamp_enforces_a_minimum_window_size() {
    let clamped = clamp_to_display(r(0, 0, 1, 1), 1280, 800);
    assert_eq!(clamped.width, MIN_WINDOW_DIM);
    assert_eq!(clamped.height, MIN_WINDOW_DIM);
}

#[test]
fn clamped_window_is_always_within_the_display() {
    // The property placement relies on: whatever comes in, the result fits.
    for &(x, y, w, h) in &[
        (0u32, 0u32, 100u32, 100u32),
        (1279, 799, 200, 200),
        (600, 400, 1280, 800),
        (50, 50, 30, 900),
    ] {
        let c = clamp_to_display(r(x, y, w, h), 1280, 800);
        assert!(c.x + c.width <= 1280, "input {x},{y},{w},{h} -> off right");
        assert!(c.y + c.height <= 800, "input {x},{y},{w},{h} -> off bottom");
        assert!(c.width >= MIN_WINDOW_DIM && c.height >= MIN_WINDOW_DIM);
    }
}
