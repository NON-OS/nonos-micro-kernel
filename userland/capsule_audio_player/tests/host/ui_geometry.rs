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

#[path = "../../src/ui/metrics.rs"]
mod metrics;

#[path = "../../src/ui/geometry.rs"]
mod geometry;

use geometry::{page, shell, Rect, SIDEBAR_W, TOPBAR_H, TRANSPORT_H};

const W: u32 = 1180;
const H: u32 = 660;

#[test]
fn shell_regions_are_disjoint_and_inside_the_surface() {
    let s = shell(W, H);
    assert_eq!(s.sidebar.w, SIDEBAR_W);
    assert_eq!(s.topbar.h, TOPBAR_H);
    assert_eq!(s.transport.h, TRANSPORT_H);
    assert_eq!(s.transport.y, H as i32 - TRANSPORT_H);
    assert_eq!(s.sidebar.bottom(), s.transport.y);
    assert_eq!(s.topbar.x, SIDEBAR_W);
    assert!(s.content.x >= SIDEBAR_W);
    assert!(s.content.y >= TOPBAR_H);
    assert!(s.content.bottom() <= s.transport.y);
    assert!(s.content.right() <= W as i32);
}

#[test]
fn the_page_stays_inside_the_content_column() {
    let s = shell(W, H);
    let p = page(&s);
    assert!(p.x >= s.content.x);
    assert!(p.y >= s.content.y);
    assert!(p.right() <= s.content.right());
    assert!(p.bottom() <= s.content.bottom());
    assert!(p.h > 400, "content column collapsed to {}", p.h);
}

#[test]
fn cells_tile_their_row_without_overlapping() {
    let r = Rect::new(10, 20, 888, 200);
    let gap = 18;
    let a = r.cell(0, 4, gap);
    let b = r.cell(1, 4, gap);
    let d = r.cell(3, 4, gap);
    assert_eq!(a.w, b.w);
    assert_eq!(b.x, a.right() + gap);
    assert!(d.right() <= r.right());
    assert_eq!(a.y, r.y);
    assert_eq!(a.h, r.h);
}

#[test]
fn contains_excludes_the_far_edges() {
    let r = Rect::new(5, 7, 30, 12);
    assert!(r.contains(5, 7));
    assert!(r.contains(34, 18));
    assert!(!r.contains(35, 18));
    assert!(!r.contains(34, 19));
    assert!(!r.contains(4, 7));
}

#[test]
fn a_narrow_surface_never_produces_a_negative_content_column() {
    for w in [320u32, 640, 800, 1180, 1920] {
        let p = page(&shell(w, 660));
        assert!(p.w >= 0, "width {w} gave {}", p.w);
        assert!(p.h >= 0, "width {w} gave {}", p.h);
    }
}
