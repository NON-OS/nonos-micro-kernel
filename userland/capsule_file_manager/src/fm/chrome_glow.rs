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

use super::theme::{GLOW, SHADE};

/// Soft cyan glow spreading outward from the rounded rect, drawn as concentric
/// hairlines that fall off quadratically. Nothing lands inside the rect, so the
/// caller may paint its plate either side of this call. Rings outside the
/// buffer are clipped away, so a full-bleed rect glows nowhere: use `glow_in`.
pub fn glow_out(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, h: u32, r: u32, spread: u32) {
    fb.shadow_round(x, y, w, h, r, spread, GLOW);
}

/// The same glow turned inward: rings start `spread` pixels inside the rect and
/// fade out at its edge. This is the only way a window-sized plate that fills
/// its whole buffer can carry a glow at all. Draw it after the plate.
pub fn glow_in(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, h: u32, r: u32, spread: u32) {
    if w <= spread * 2 || h <= spread * 2 {
        return;
    }
    fb.shadow_round(
        x + spread,
        y + spread,
        w - spread * 2,
        h - spread * 2,
        r.saturating_sub(spread),
        spread,
        GLOW,
    );
}

/// Neutral drop shade under a raised plate: depth without spending the one hue.
pub fn shade_out(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, h: u32, r: u32, spread: u32) {
    fb.shadow_round(x, y, w, h, r, spread, SHADE);
}
