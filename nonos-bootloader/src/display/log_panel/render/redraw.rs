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

use super::draw::draw_entry_at;
use crate::display::log_panel::buffer::get_count;
use crate::display::log_panel::types::{max_visible_lines, MAX_LOG_LINES};

fn redraw_all_visible(total: usize) {
    if total == 0 {
        return;
    }
    let max_lines = max_visible_lines();
    let visible_count = total.min(max_lines);
    let start_entry = if total > max_lines { total - max_lines } else { 0 };
    for line in 0..visible_count {
        let entry_idx = (start_entry + line) % MAX_LOG_LINES;
        draw_entry_at(line, entry_idx);
    }
}

pub fn redraw_all() {
    let count = get_count();
    if count > 0 {
        redraw_all_visible(count);
    }
}

fn log_delay() {
    for _ in 0..300_000 {
        core::hint::spin_loop();
    }
}

pub fn render_after_log(count: usize) {
    if count == 0 {
        return;
    }
    let max_lines = max_visible_lines();
    if count > max_lines {
        redraw_all_visible(count);
    } else {
        let line_num = count - 1;
        draw_entry_at(line_num, line_num);
    }
    log_delay();
}
