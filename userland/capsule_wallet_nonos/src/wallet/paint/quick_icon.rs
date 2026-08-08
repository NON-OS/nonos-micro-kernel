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

/// Which quick action an icon stands for.
pub enum Icon {
    Send,
    Receive,
    Stake,
    Swap,
}

/// Draw an action icon inside a 34 by 34 tile at (x, y).
///
/// Drawn from rectangles rather than set as text. The arrows were briefly
/// font glyphs, which rendered as empty boxes because the bundled face has no
/// arrow codepoints, and before that they were ASCII stand-ins that read as
/// typos. Shapes always render, at any size, on any face.
pub fn icon(fb: &mut PaintBuffer, x: u32, y: u32, kind: Icon, c: u32) {
    let cx = x + 17;
    let cy = y + 17;
    match kind {
        Icon::Send => arrow(fb, cx, cy, c, true),
        Icon::Receive => arrow(fb, cx, cy, c, false),
        Icon::Stake => {
            // A stack: three bars, the way a staked balance is drawn anywhere.
            for i in 0..3u32 {
                fb.fill_rect(cx - 7, cy - 6 + i * 5, 14, 3, c);
            }
        }
        Icon::Swap => {
            // Two lanes running opposite ways, each with a head at its end.
            fb.fill_rect(cx - 8, cy - 4, 14, 2, c);
            fb.fill_rect(cx - 6, cy + 3, 14, 2, c);
            head(fb, cx + 6, cy - 3, c, true);
            head(fb, cx - 6, cy + 4, c, false);
        }
    }
}

// A vertical arrow: stem plus a triangular head, up or down.
fn arrow(fb: &mut PaintBuffer, cx: u32, cy: u32, c: u32, up: bool) {
    fb.fill_rect(cx - 1, cy - 7, 3, 14, c);
    for i in 0..5u32 {
        let w = 1 + i * 2;
        let ry = if up { cy - 7 + i } else { cy + 6 - i };
        fb.fill_rect(cx - i, ry, w, 1, c);
    }
}

// A small horizontal arrow head, pointing right or left.
fn head(fb: &mut PaintBuffer, x: u32, y: u32, c: u32, right: bool) {
    for i in 0..4u32 {
        let rx = if right { x - i } else { x + i };
        fb.fill_rect(rx, y.saturating_sub(i) + i, 1, 1 + i * 2, c);
    }
}
