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

#[path = "../../src/ui/art/hue.rs"]
mod hue;

use hue::{hsl, hue, motif};

const IDS: [&str; 8] = [
    "/audio/one.wav",
    "/audio/two.wav",
    "/audio/three.mp3",
    "/audio/four.mp3",
    "",
    "/audio/a.wav",
    "/audio/b.wav",
    "/audio/nested/deep/track.mp3",
];

#[test]
fn a_track_always_gets_the_same_hue() {
    for id in IDS {
        assert_eq!(hue(id), hue(id));
        assert_eq!(motif(id), motif(id));
    }
}

#[test]
fn every_hue_lands_in_the_cyan_or_violet_band() {
    for id in IDS {
        let h = hue(id);
        let cyan = (170..=215).contains(&h);
        let violet = (265..=315).contains(&h);
        assert!(cyan || violet, "{id} gave {h}, outside both bands");
    }
}

#[test]
fn motifs_stay_inside_the_dispatch_table() {
    for id in IDS {
        assert!(motif(id) < 6, "{id} gave motif {}", motif(id));
    }
}

#[test]
fn different_ids_do_not_all_collapse_to_one_hue() {
    let mut seen = IDS.iter().map(|id| hue(id)).collect::<Vec<_>>();
    seen.sort_unstable();
    seen.dedup();
    assert!(seen.len() > 2, "only {} distinct hues across {} ids", seen.len(), IDS.len());
}

#[test]
fn hsl_stays_inside_the_channel_range() {
    for h in (0..360).step_by(17) {
        for l in [0u32, 5, 16, 50, 66, 100] {
            let c = hsl(h, 92, l);
            assert_eq!(c >> 24, 0, "hsl({h},92,{l}) set bits above the channels");
        }
    }
}
