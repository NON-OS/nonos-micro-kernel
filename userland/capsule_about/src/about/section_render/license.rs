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

use super::row;
use crate::about::data::license::{NAME, TEXT, URL, VERSION};

const HEADER_ROWS: u32 = 4;

pub fn line_count() -> u32 {
    HEADER_ROWS + TEXT.lines().count() as u32
}

pub fn render(scroll: u32, visible: u32, top: u32, fb: &mut PaintBuffer) {
    let total = line_count();
    let end = scroll.saturating_add(visible).min(total);
    let mut idx: u32 = 0;
    let mut visible_idx: u32 = 0;
    let header: [(&[u8], &[u8]); 3] = [(b"Name", NAME), (b"Version", VERSION), (b"URL", URL)];
    for (label, value) in header {
        if idx >= scroll && idx < end {
            row::pair(label, value, row::line_y(visible_idx, top), fb);
            visible_idx += 1;
        }
        idx += 1;
    }
    if idx >= scroll && idx < end {
        row::single(b"", row::line_y(visible_idx, top), fb);
        visible_idx += 1;
    }
    idx += 1;
    for line in TEXT.lines() {
        if idx >= scroll && idx < end {
            row::single(line.as_bytes(), row::line_y(visible_idx, top), fb);
            visible_idx += 1;
        }
        idx += 1;
    }
}
