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

use alloc::vec::Vec;

use super::icon_path::Icon;

// The strip is 236px inside its padding, which is under two measured labels
// wide at the 17px floor `MIN_UI_PX` clamps to, so the tiles carry glyphs only
// and every one of them names itself in the status line when it fires.
const TILE_GAP: u32 = 4;

/// A quick action on whatever the info panel is describing. All six are wired;
/// each one lands on a handler this crate already has.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Quick {
    Open,
    Copy,
    Duplicate,
    Pin,
    Tag,
    Delete,
}

pub const QUICKS: [(Quick, Icon); 6] = [
    (Quick::Open, Icon::Forward),
    (Quick::Copy, Icon::Doc),
    (Quick::Duplicate, Icon::Plus),
    (Quick::Pin, Icon::Star),
    (Quick::Tag, Icon::Tag),
    (Quick::Delete, Icon::Trash),
];

/// One tile's box.
pub struct QuickSlot {
    pub x: u32,
    pub w: u32,
    pub action: Quick,
    pub icon: Icon,
}

/// The six tiles spread evenly across `w`, in the one pass the painter draws
/// from and the hit-test resolves against.
pub fn quick_slots(x: u32, w: u32) -> Vec<QuickSlot> {
    let n = QUICKS.len() as u32;
    let tile = w.saturating_sub(TILE_GAP * (n - 1)) / n;
    let mut out = Vec::new();
    for (i, (action, icon)) in QUICKS.into_iter().enumerate() {
        out.push(QuickSlot { x: x + (tile + TILE_GAP) * i as u32, w: tile, action, icon });
    }
    out
}
