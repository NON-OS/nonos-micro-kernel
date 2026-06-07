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

use core::sync::atomic::{AtomicU8, Ordering};

use crate::display::constants::{COLOR_ACCENT, COLOR_BORDER, COLOR_PROGRESS_BG};
use crate::display::gop::{draw_rect, fill_rect, get_dimensions, is_initialized};

static CURRENT_STAGE: AtomicU8 = AtomicU8::new(0);
static TOTAL_STAGES: AtomicU8 = AtomicU8::new(10);

pub fn draw_boot_progress(current: u8, total: u8) {
    CURRENT_STAGE.store(current, Ordering::Relaxed);
    TOTAL_STAGES.store(total, Ordering::Relaxed);
    render_progress_bar();
}

pub fn get_progress() -> (u8, u8) {
    (CURRENT_STAGE.load(Ordering::Relaxed), TOTAL_STAGES.load(Ordering::Relaxed))
}

fn render_progress_bar() {
    if !is_initialized() {
        return;
    }
    let (w, h) = get_dimensions();
    let (cur, total) = get_progress();
    let pct = if total == 0 { 0 } else { (cur as u32 * 100) / total as u32 };
    let bar_w = w.saturating_sub(80);
    let x = 40;
    let y = h.saturating_sub(24);
    fill_rect(x, y, bar_w, 10, COLOR_PROGRESS_BG);
    draw_rect(x, y, bar_w, 10, COLOR_BORDER);
    fill_rect(x + 1, y + 1, bar_w.saturating_sub(2) * pct / 100, 8, COLOR_ACCENT);
}
