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

use super::constants::{CELL_WIDTH, HEADER_H};
use crate::term::state::State;
use crate::term::theme::{ACCENT, HEADER_BG, PATH};

pub const TAB_W: u32 = 16 * CELL_WIDTH;
pub const STRIP_Y: u32 = HEADER_H;
pub const STRIP_H: u32 = 16;
pub const CLOSE_W: u32 = 14;
pub const PLUS_W: u32 = 2 * CELL_WIDTH;

pub fn draw_tabstrip(tabs: &[State], active: usize, fb: &mut PaintBuffer) {
    fb.fill_rect(0, STRIP_Y, fb.width, STRIP_H, HEADER_BG);
    for (i, tab) in tabs.iter().enumerate() {
        let x = i as u32 * TAB_W;
        if i == active {
            fb.fill_rect(x, STRIP_Y, TAB_W, STRIP_H, ACCENT);
        }
        let mut label = [b' '; 12];
        label[0] = b'1' + i as u8;
        label[1] = b':';
        let base = basename(tab.cwd.as_bytes());
        let take = base.len().min(9);
        label[3..3 + take].copy_from_slice(&base[base.len() - take..]);
        let fg = if i == active { HEADER_BG } else { PATH };
        fb.text(x + 4, STRIP_Y + 2, &label[..3 + take], fg);
        fb.text(x + TAB_W - CLOSE_W, STRIP_Y + 2, b"x", fg);
    }
    let px = tabs.len() as u32 * TAB_W;
    fb.text(px + 6, STRIP_Y + 2, b"+", ACCENT);
}

fn basename(path: &[u8]) -> &[u8] {
    match path.iter().rposition(|&b| b == b'/') {
        Some(i) if i + 1 < path.len() => &path[i + 1..],
        _ => path,
    }
}
