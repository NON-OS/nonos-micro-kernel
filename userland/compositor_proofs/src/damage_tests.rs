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

//! The one property the whole "only repaint what changed" scheme rests on: the
//! rectangles the accumulator hands back must cover every pixel that was ever
//! accumulated. If a merge ever dropped coverage, the compositor would leave
//! stale pixels on screen — exactly the cursor-trail class of bug. These tests
//! drive the real accumulator and check coverage at pixel granularity.

use std::collections::HashSet;

use crate::damage::{DamageAccumulator, Rect};

fn cells(r: Rect) -> Vec<(u32, u32)> {
    let mut v = Vec::new();
    for y in r.y..r.y + r.height {
        for x in r.x..r.x + r.width {
            v.push((x, y));
        }
    }
    v
}

// Accumulate every input, drain everything, and assert the drained rectangles
// cover every cell of every input.
fn assert_covers(inputs: &[Rect]) {
    let mut acc = DamageAccumulator::new();
    let mut wanted: HashSet<(u32, u32)> = HashSet::new();
    for &r in inputs {
        if r.width != 0 && r.height != 0 {
            wanted.extend(cells(r));
        }
        acc.accumulate(r);
    }
    let mut covered: HashSet<(u32, u32)> = HashSet::new();
    while let Some(out) = acc.drain() {
        covered.extend(cells(out));
    }
    let missing: Vec<_> = wanted.difference(&covered).collect();
    assert!(missing.is_empty(), "uncovered cells: {:?}", missing);
}

#[test]
fn two_separate_rects_are_both_covered() {
    // The cursor-move case: an old position and a far new position.
    assert_covers(&[
        Rect { x: 10, y: 10, width: 20, height: 20 },
        Rect { x: 400, y: 300, width: 20, height: 20 },
    ]);
}

#[test]
fn touching_rects_merge_without_losing_coverage() {
    assert_covers(&[
        Rect { x: 0, y: 0, width: 16, height: 16 },
        Rect { x: 16, y: 0, width: 16, height: 16 },
        Rect { x: 0, y: 16, width: 16, height: 16 },
    ]);
}

#[test]
fn overflow_past_capacity_still_covers_everything() {
    // More than MAX_RECTS distinct regions forces the smallest-growth merge;
    // coverage must survive it. Twelve well-separated 8x8 squares.
    let mut inputs = Vec::new();
    for i in 0..12u32 {
        inputs.push(Rect { x: i * 40, y: i * 30, width: 8, height: 8 });
    }
    assert_covers(&inputs);
}

#[test]
fn mark_full_then_accumulate_covers_the_screen() {
    let mut acc = DamageAccumulator::new();
    acc.mark_full(64, 48);
    acc.accumulate(Rect { x: 10, y: 10, width: 4, height: 4 });
    let mut covered: HashSet<(u32, u32)> = HashSet::new();
    while let Some(out) = acc.drain() {
        covered.extend(cells(out));
    }
    for y in 0..48 {
        for x in 0..64 {
            assert!(covered.contains(&(x, y)), "screen cell {x},{y} not covered");
        }
    }
}

#[test]
fn zero_sized_rects_are_ignored() {
    let mut acc = DamageAccumulator::new();
    acc.accumulate(Rect { x: 5, y: 5, width: 0, height: 10 });
    acc.accumulate(Rect { x: 5, y: 5, width: 10, height: 0 });
    assert!(acc.drain().is_none(), "a zero-area rect must not become work");
}

#[test]
fn many_cursor_moves_never_drop_the_old_position() {
    // Simulate a cursor sweeping across the screen: each step damages the old
    // and new 32x32 box. Every box must end up covered even as merges happen.
    let mut inputs = Vec::new();
    let (mut px, mut py) = (100u32, 100u32);
    for i in 0..20u32 {
        let (nx, ny) = (100 + i * 17, 100 + i * 11);
        inputs.push(Rect { x: px, y: py, width: 32, height: 32 });
        inputs.push(Rect { x: nx, y: ny, width: 32, height: 32 });
        px = nx;
        py = ny;
    }
    assert_covers(&inputs);
}
