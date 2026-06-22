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

use crate::display::fx::clear_region;
use crate::display::log_panel::types::{
    get_log_area, line_clear_width, max_visible_lines, LINE_HEIGHT,
};

pub fn clear_line(line_num: usize) {
    let (log_x, log_y) = get_log_area();
    let y = log_y + (line_num as u32) * LINE_HEIGHT;
    clear_region(log_x.saturating_sub(24), y, line_clear_width() + 24, LINE_HEIGHT);
}

pub fn clear_display() {
    let (log_x, log_y) = get_log_area();
    let height = (max_visible_lines() as u32) * LINE_HEIGHT;
    clear_region(log_x.saturating_sub(24), log_y, line_clear_width() + 24, height);
}
