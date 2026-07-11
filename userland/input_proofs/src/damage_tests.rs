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

//! Proofs for the compositor damage accumulator: far-apart damage stays split
//! (the performance win), touching damage merges (no double work), and every
//! accumulated pixel is always covered by what is drained (no tearing).

use crate::damage::{DamageAccumulator, Rect};

fn r(x: u32, y: u32, w: u32, h: u32) -> Rect {
    Rect { x, y, width: w, height: h }
}

fn drain_all(acc: &mut DamageAccumulator) -> Vec<Rect> {
    let mut out = Vec::new();
    while let Some(rect) = acc.drain() {
        out.push(rect);
    }
    out
}

fn covered(rects: &[Rect], px: u32, py: u32) -> bool {
    rects.iter().any(|r| px >= r.x && px < r.x + r.width && py >= r.y && py < r.y + r.height)
}

// The whole point: two far-apart regions do not merge into one big box.
#[test]
fn distant_damage_stays_separate() {
    let mut acc = DamageAccumulator::new();
    acc.accumulate(r(0, 0, 10, 10));
    acc.accumulate(r(500, 400, 10, 10));
    let rects = drain_all(&mut acc);
    assert_eq!(rects.len(), 2);
    let area: u64 = rects.iter().map(|r| r.width as u64 * r.height as u64).sum();
    assert_eq!(area, 200); // not 510*410 as a merged box would be
}

// Overlapping damage merges so the shared pixels are composited once.
#[test]
fn overlapping_damage_merges() {
    let mut acc = DamageAccumulator::new();
    acc.accumulate(r(0, 0, 20, 20));
    acc.accumulate(r(10, 10, 20, 20));
    let rects = drain_all(&mut acc);
    assert_eq!(rects.len(), 1);
    assert_eq!((rects[0].x, rects[0].y, rects[0].width, rects[0].height), (0, 0, 30, 30));
}

// No accumulated pixel is ever left uncovered, even past the rect-list cap.
#[test]
fn every_pixel_stays_covered_when_full() {
    let mut acc = DamageAccumulator::new();
    let inputs: Vec<Rect> = (0..20).map(|i| r(i * 30, i * 20, 8, 8)).collect();
    for rect in &inputs {
        acc.accumulate(*rect);
    }
    let rects = drain_all(&mut acc);
    for src in &inputs {
        assert!(covered(&rects, src.x, src.y));
        assert!(covered(&rects, src.x + src.width - 1, src.y + src.height - 1));
    }
}

// mark_full replaces the set with one screen-sized rect.
#[test]
fn mark_full_covers_everything() {
    let mut acc = DamageAccumulator::new();
    acc.accumulate(r(5, 5, 2, 2));
    acc.mark_full(800, 600);
    let rects = drain_all(&mut acc);
    assert_eq!(rects.len(), 1);
    assert_eq!((rects[0].width, rects[0].height), (800, 600));
}

// Zero-area damage is ignored.
#[test]
fn empty_damage_is_dropped() {
    let mut acc = DamageAccumulator::new();
    acc.accumulate(r(10, 10, 0, 5));
    acc.accumulate(r(10, 10, 5, 0));
    assert!(drain_all(&mut acc).is_empty());
}
