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

use crate::about::theme::{ACCENT, TRACK_BG};

use super::super::metrics::{CARD_PAD, HERO_H, HERO_MARK_R, HERO_MARK_T};

// The ring reads as a seal and the stroke through it as the slash in the wordmark.
// Both are integer primitives centred on the same point, so the diagonal always
// meets the annulus at the same two places whatever the ring radius becomes.
pub fn mark(fb: &mut PaintBuffer, y: i32) {
    let cy = y + (HERO_H / 2) as i32;
    if cy < (HERO_MARK_R + 8) as i32 || cy + (HERO_MARK_R + 8) as i32 >= fb.height as i32 {
        return;
    }
    let cx = CARD_PAD + HERO_MARK_R + 8;
    let cy = cy as u32;
    fb.ring(cx, cy, HERO_MARK_R + 7, 1, TRACK_BG);
    fb.ring(cx, cy, HERO_MARK_R, HERO_MARK_T, ACCENT);
    let r = (HERO_MARK_R - 8) as i32;
    fb.line_aa(cx as i32 - r, cy as i32 + r, cx as i32 + r, cy as i32 - r, ACCENT);
}
