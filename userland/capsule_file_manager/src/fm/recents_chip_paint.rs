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

use nonos_app_skeleton::PaintBuffer;

use super::chip::{chip, CHIP_H};
use super::filetype::Kind;
use super::recents_chips::KindChip;

/// The chip covering `(x, y)`: the outer option says a chip was hit, the inner
/// one is the filter it selects.
pub fn chip_at(chips: &[KindChip], x: u32, y: u32) -> Option<Option<Kind>> {
    chips
        .iter()
        .find(|c| x >= c.x && x < c.x + c.w && y >= c.y && y < c.y + CHIP_H)
        .map(|c| c.kind)
}

/// Draws the row from the same slots the hit-test walks.
pub fn paint_chips(fb: &mut PaintBuffer, chips: &[KindChip], active: Option<Kind>) {
    for c in chips {
        chip(fb, c.x, c.y, c.label.as_str(), c.kind == active);
    }
}
