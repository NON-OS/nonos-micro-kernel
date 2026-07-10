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

//! Icon-grid view: the current directory drawn as large cyan folder and file
//! icons with a centred label, the selected cell lit on a rounded tile. The
//! columns and starting cell come from state so a click lands on what was drawn.

use nonos_app_skeleton::PaintBuffer;

use super::icon;
use super::layout::{CONTENT_X, GRID_CELL_H, GRID_CELL_W, GRID_ICON, GRID_PAD_X, GRID_TOP};
use super::state::State;
use super::theme::{BACKGROUND, FILE_C, FOREGROUND, SELECTED, SELECT_BG};

const FOLDER: u32 = 0xFF5F_E3E8;
const LABEL_PX: f32 = 13.0;
const MAX_LABEL: usize = 16;

pub fn paint_grid(state: &State, fb: &mut PaintBuffer) {
    let cols = state.grid_cols.max(1) as usize;
    // Align the first drawn cell to a row boundary so wrapping stays stable.
    let start = state.scroll - (state.scroll % cols);
    let left = CONTENT_X + GRID_PAD_X / 2;

    for (vis, idx) in (start..state.entries.len()).take(state.view_rows).enumerate() {
        let entry = &state.entries[idx];
        let col = (vis % cols) as u32;
        let row = (vis / cols) as u32;
        let cx = left + col * GRID_CELL_W;
        let cy = GRID_TOP + row * GRID_CELL_H;

        let selected = idx == state.cursor || state.selected.iter().any(|s| s == &entry.full_path);
        let cell_bg = if selected {
            fill_round(fb, cx + 6, cy + 2, GRID_CELL_W - 12, GRID_CELL_H - 8, SELECT_BG);
            SELECT_BG
        } else {
            BACKGROUND
        };

        let icon_x = cx + (GRID_CELL_W - GRID_ICON) / 2;
        let icon_y = cy + 10;
        if entry.is_dir {
            icon::folder(fb, icon_x, icon_y, GRID_ICON, FOLDER, cell_bg);
        } else {
            icon::file(fb, icon_x, icon_y, GRID_ICON, FILE_C, cell_bg);
        }

        let name = trim_label(&entry.label);
        let tw = fb.measure_ttf(name, LABEL_PX).max(0) as u32;
        let lx = cx + GRID_CELL_W.saturating_sub(tw) / 2;
        let ly = icon_y + GRID_ICON + 8;
        let color = if selected { SELECTED } else { FOREGROUND };
        let _ = fb.text_ttf(lx as i32, ly as i32, name, color, LABEL_PX);
    }
}

// Clip a label to a whole number of characters so it never splits a UTF-8 code
// point, and drop the trailing slash directories carry.
fn trim_label(label: &str) -> &str {
    let l = label.trim_end_matches('/');
    match l.char_indices().nth(MAX_LABEL) {
        Some((i, _)) => &l[..i],
        None => l,
    }
}

// A filled rectangle with the corners knocked back so the selection reads as a
// rounded tile rather than a hard box.
fn fill_round(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, h: u32, c: u32) {
    fb.fill_rect(x, y, w, h, c);
    for &(cx, cy) in &[(x, y), (x + w - 2, y), (x, y + h - 2), (x + w - 2, y + h - 2)] {
        fb.fill_rect(cx, cy, 2, 2, BACKGROUND);
    }
}
