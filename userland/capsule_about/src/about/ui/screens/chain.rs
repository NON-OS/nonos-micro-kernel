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

use crate::about::theme::{ACCENT, FOREGROUND, PILL_BG, PILL_BORDER, RULE};

use super::super::metrics::{BODY_PX, CHAIN_H, CHAIN_LINK, CHIP_PAD_X, CHIP_RADIUS};
use super::super::text::{self, line, top_of};

fn pill_w(label: &[u8]) -> u32 {
    CHIP_PAD_X * 2 + text::width_of(label, BODY_PX)
}

// A left-to-right chain of pills joined by hairlines. The last node is the thing
// the chain is about, so it carries the accent and the ones before it do not.
pub fn paint(fb: &mut PaintBuffer, x: u32, y: i32, nodes: &[&[u8]]) {
    let mut cx = x;
    let last = nodes.len().saturating_sub(1);
    for (i, node) in nodes.iter().enumerate() {
        let w = pill_w(node);
        let fg = if i == last { ACCENT } else { FOREGROUND };
        if y >= 0 && y + CHAIN_H as i32 <= fb.height as i32 {
            fb.fill_round(cx, y as u32, w, CHAIN_H, CHIP_RADIUS, PILL_BG);
            fb.stroke_round(cx, y as u32, w, CHAIN_H, CHIP_RADIUS, 1, PILL_BORDER);
        }
        line(fb, cx + CHIP_PAD_X, top_of(y, CHAIN_H, BODY_PX), node, fg, BODY_PX);
        cx += w;
        if i < last {
            link(fb, cx, y);
            cx += CHAIN_LINK;
        }
    }
}

fn link(fb: &mut PaintBuffer, x: u32, y: i32) {
    let mid = y + (CHAIN_H / 2) as i32;
    if mid < 0 || mid >= fb.height as i32 {
        return;
    }
    fb.fill_rect(x, mid as u32, CHAIN_LINK, 1, RULE);
    fb.circle(x + CHAIN_LINK - 3, mid as u32, 3, RULE);
}
