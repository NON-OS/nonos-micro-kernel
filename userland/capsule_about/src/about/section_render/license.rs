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
use crate::about::data::license::{NAME, SUMMARY_LINES, URL, VERSION};
use crate::about::state::VISIBLE_BODY_LINES;

const FIXED_ROWS: u32 = 4;

pub fn line_count() -> u32 {
    FIXED_ROWS + SUMMARY_LINES.len() as u32
}

pub fn render(scroll: u32, top: u32, fb: &mut PaintBuffer) {
    let total = line_count();
    let end = (scroll + VISIBLE_BODY_LINES).min(total);
    let mut idx: u32 = 0;
    let mut visible: u32 = 0;
    let header_pairs: [(&[u8], &[u8]); 3] = [(b"Name", NAME), (b"Version", VERSION), (b"URL", URL)];
    for (label, value) in header_pairs {
        if idx >= scroll && idx < end {
            row::pair(label, value, row::line_y(visible, top), fb);
            visible += 1;
        }
        idx += 1;
    }
    if idx >= scroll && idx < end {
        row::single(b"", row::line_y(visible, top), fb);
        visible += 1;
    }
    idx += 1;
    for line in SUMMARY_LINES {
        if idx >= scroll && idx < end {
            row::single(line, row::line_y(visible, top), fb);
            visible += 1;
        }
        idx += 1;
    }
}
