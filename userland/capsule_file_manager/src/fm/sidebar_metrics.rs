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

use super::layout::SIDEBAR_W;

// The rail's own metrics. Every plate on it -- an active row's pill, the brand
// mark, the drive card -- is inset by the same amount from both edges, so the
// column reads as one stack rather than as three differently-margined things.
pub const RAIL_X: u32 = 8;
pub const RAIL_PAD: u32 = 12;

// Icons are stroked line art: at 18px the fine detail collapsed, so the rail
// carries them at 22 and the label column starts past that.
pub const ICON_S: u32 = 22;
pub const ICON_GAP: u32 = 12;

/// Width of a full-bleed plate on the rail.
pub const fn rail_w() -> u32 {
    SIDEBAR_W - RAIL_X * 2
}

/// Left edge of the content inside such a plate.
pub const fn rail_text_x() -> u32 {
    RAIL_X + RAIL_PAD + ICON_S + ICON_GAP
}

/// The height a single line of `px` text occupies, used to centre a run against
/// an icon of a known size. The toolkit's real line metrics are not exported to
/// capsules, so this is the same 1.25em box the rest of the chrome assumes.
pub fn line_box(px: f32) -> u32 {
    let clamped = if px < MIN_PX { MIN_PX } else { px };
    (clamped * 1.25) as u32
}

/// The floor the font layer clamps every draw and every measure up to. Stated
/// here because layout that ignores it centres text against the size it asked
/// for rather than the size that lands on screen.
pub const MIN_PX: f32 = 17.0;
