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
pub mod metrics;

#[path = "../../src/ui/geometry.rs"]
pub mod geometry;

mod ui {
    pub use super::{geometry, metrics};
}

#[path = "../../src/ui/shell/bar.rs"]
mod bar;

use bar::{bar, PLAY_R, THUMB};
use geometry::{shell, Rect};

fn transport_rect(w: u32, h: u32) -> Rect {
    shell(w, h).transport
}

fn controls(r: Rect) -> Vec<(&'static str, Rect)> {
    let b = bar(r);
    vec![
        ("thumb", b.thumb),
        ("shuffle", b.shuffle),
        ("prev", b.prev),
        ("play", b.play),
        ("next", b.next),
        ("repeat", b.repeat),
        ("scrub", b.scrub),
        ("speaker", b.speaker),
        ("volume", b.volume),
    ]
}

fn overlaps(a: Rect, b: Rect) -> bool {
    a.x < b.right() && b.x < a.right() && a.y < b.bottom() && b.y < a.bottom()
}

#[test]
fn every_control_sits_inside_the_transport_bar() {
    let r = transport_rect(1180, 660);
    for (name, c) in controls(r) {
        assert!(c.x >= r.x, "{name} starts left of the bar");
        assert!(c.right() <= r.right(), "{name} runs past the right edge");
        assert!(c.y >= r.y, "{name} sits above the bar");
        assert!(c.bottom() <= r.bottom(), "{name} sits below the bar");
    }
}

#[test]
fn no_two_controls_overlap() {
    let r = transport_rect(1180, 660);
    let cs = controls(r);
    for i in 0..cs.len() {
        for j in (i + 1)..cs.len() {
            assert!(!overlaps(cs[i].1, cs[j].1), "{} overlaps {}", cs[i].0, cs[j].0);
        }
    }
}

#[test]
fn the_cluster_reads_left_to_right() {
    let b = bar(transport_rect(1180, 660));
    assert!(b.shuffle.right() <= b.prev.x);
    assert!(b.prev.right() <= b.play.x);
    assert!(b.play.right() <= b.next.x);
    assert!(b.next.right() <= b.repeat.x);
    assert!(b.repeat.right() <= b.scrub.x);
    assert!(b.scrub.right() <= b.speaker.x);
    assert!(b.speaker.right() <= b.volume.x);
}

#[test]
fn the_play_disc_and_thumb_keep_their_stated_size() {
    let b = bar(transport_rect(1180, 660));
    assert_eq!(b.play.w, PLAY_R * 2);
    assert_eq!(b.play.h, PLAY_R * 2);
    assert_eq!(b.thumb.w, THUMB);
    assert_eq!(b.thumb.h, THUMB);
}

#[test]
fn the_scrubber_never_inverts_on_a_narrow_surface() {
    for w in [800u32, 960, 1180, 1440, 1920] {
        let b = bar(transport_rect(w, 660));
        assert!(b.scrub.w >= 0, "width {w} gave scrub {}", b.scrub.w);
        assert!(b.volume.right() <= w as i32, "width {w} pushed volume off the edge");
    }
}
