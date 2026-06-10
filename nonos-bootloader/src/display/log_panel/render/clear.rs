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

use crate::display::gop::fill_rect;
use crate::display::log_panel::types::{
    get_log_area, max_visible_lines, LINE_HEIGHT, LOG_LINE_LEN,
};

const BG_COLOR: u32 = 0xFF000000;
const LOG_LINE_WIDTH: u32 = (LOG_LINE_LEN as u32 + 4) * 8;

pub fn clear_line(line_num: usize) {
    let (log_x, log_y) = get_log_area();
    let y = log_y + (line_num as u32) * LINE_HEIGHT;
    fill_rect(log_x, y, LOG_LINE_WIDTH, LINE_HEIGHT, BG_COLOR);
}

pub fn clear_display() {
    let (log_x, log_y) = get_log_area();
    let height = (max_visible_lines() as u32) * LINE_HEIGHT;
    fill_rect(log_x, log_y, LOG_LINE_WIDTH, height, BG_COLOR);
}
