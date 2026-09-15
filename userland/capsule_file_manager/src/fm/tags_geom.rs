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

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use super::chip::{chip_w, CHIP_GAP, CHIP_H};
use super::layout::{CONTENT_X, HEADER_H, PAD_X, SECTION_GAP};
use super::screen_row::LABEL_ADV;
use super::state::State;
use super::tags::TagMap;

/// One laid-out tag pill: the painter draws these and `chip_at` searches them,
/// so a filter chip is only ever clickable where it was actually drawn.
pub struct ChipSlot {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub name: String,
}

/// Every tag in use, once, in first-seen order. The map is keyed by path, so
/// this is the only place the tag axis is materialised.
pub fn tag_names(tags: &TagMap) -> Vec<&str> {
    let mut out: Vec<&str> = Vec::new();
    for (_, names) in tags.entries.iter() {
        for name in names {
            if !out.iter().any(|t| *t == name.as_str()) {
                out.push(name.as_str());
            }
        }
    }
    out
}

/// The wrapped chip row. A pill moves to the next line when the measured one
/// would overrun the content width.
pub fn chip_slots(state: &State) -> Vec<ChipSlot> {
    let left = CONTENT_X + PAD_X;
    let right = left + state.win_w.saturating_sub(CONTENT_X + PAD_X * 2);
    let mut out = Vec::new();
    let (mut x, mut y) = (left, chips_top());
    for name in tag_names(&state.tags) {
        let w = chip_w(name);
        if x > left && x + w > right {
            x = left;
            y += CHIP_H + CHIP_GAP;
        }
        out.push(ChipSlot { x, y, w, name: String::from(name) });
        x += w + CHIP_GAP;
    }
    out
}

/// Top of the chip row, below the section label.
pub fn chips_top() -> u32 {
    HEADER_H + 16 + LABEL_ADV
}

/// The y just past the chip row, where the filtered listing starts.
pub fn chips_bottom(state: &State) -> u32 {
    chip_slots(state).last().map(|s| s.y).unwrap_or(chips_top()) + CHIP_H + SECTION_GAP
}

/// The tag whose pill covers `(x, y)`.
pub fn chip_at(state: &State, x: u32, y: u32) -> Option<String> {
    chip_slots(state)
        .into_iter()
        .find(|s| x >= s.x && x < s.x + s.w && y >= s.y && y < s.y + CHIP_H)
        .map(|s| s.name)
}
