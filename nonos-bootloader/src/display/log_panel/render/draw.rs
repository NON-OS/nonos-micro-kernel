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

use super::clear::clear_line;
use crate::display::font::draw_string;
use crate::display::fx::blend_rect;
use crate::display::gop::fill_rect;
use crate::display::log_panel::buffer::get_entry;
use crate::display::log_panel::types::{get_log_area, line_clear_width, LogLevel, LINE_HEIGHT};

const CYAN: u32 = 0xFF00F5D4;
const TEXT_DONE: u32 = 0xFFBED6D4;
const DIM: u32 = 0xFF4F666A;
const AMBER: u32 = 0xFFE8A33D;
const RED: u32 = 0xFFE0554A;

pub fn draw_entry_at(line_num: usize, entry_idx: usize) {
    let (log_x, log_y) = get_log_area();
    let y = log_y + (line_num as u32) * LINE_HEIGHT;
    clear_line(line_num);
    if let Some(entry) = get_entry(entry_idx) {
        if entry.len == 0 {
            return;
        }
        let (mark, text, led) = match entry.level {
            LogLevel::Ok => (CYAN, TEXT_DONE, true),
            LogLevel::Info => (DIM, DIM, false),
            LogLevel::Warn => (AMBER, AMBER, true),
            LogLevel::Error => (RED, RED, true),
            LogLevel::Security => (CYAN, CYAN, true),
        };
        if led {
            blend_rect(log_x.saturating_sub(1), y + 3, 11, 11, mark, 22);
            fill_rect(log_x + 1, y + 5, 7, 7, mark);
        } else {
            fill_rect(log_x + 3, y + 7, 3, 3, mark);
        }
        let max_chars = (line_clear_width().saturating_sub(40) / 8) as usize;
        draw_string(log_x + 24, y, &entry.text[..entry.len.min(max_chars)], text);
    }
}
