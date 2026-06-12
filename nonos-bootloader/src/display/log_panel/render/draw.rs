// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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
use crate::display::constants::{COLOR_ACCENT, COLOR_ERROR, COLOR_SUCCESS, COLOR_WARNING};
use crate::display::font::draw_string;
use crate::display::log_panel::buffer::get_entry;
use crate::display::log_panel::types::{get_log_area, line_clear_width, LogLevel, LINE_HEIGHT};

const COLOR_INFO: u32 = 0xFF6B7280;

pub fn draw_entry_at(line_num: usize, entry_idx: usize) {
    let (log_x, log_y) = get_log_area();
    let y = log_y + (line_num as u32) * LINE_HEIGHT;
    clear_line(line_num);
    if let Some(entry) = get_entry(entry_idx) {
        if entry.len == 0 {
            return;
        }
        let (prefix, color) = match entry.level {
            LogLevel::Info => (b"    " as &[u8], COLOR_INFO),
            LogLevel::Ok => (b"[+] " as &[u8], COLOR_SUCCESS),
            LogLevel::Warn => (b"[!] " as &[u8], COLOR_WARNING),
            LogLevel::Error => (b"[X] " as &[u8], COLOR_ERROR),
            LogLevel::Security => (b"[S] " as &[u8], COLOR_ACCENT),
        };
        let max_chars = (line_clear_width().saturating_sub(40) / 8) as usize;
        draw_string(log_x, y, prefix, color);
        draw_string(log_x + 32, y, &entry.text[..entry.len.min(max_chars)], color);
    }
}
