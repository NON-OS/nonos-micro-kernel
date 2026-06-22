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

use core::sync::atomic::{AtomicU8, Ordering};

use super::layout::splash;
use super::vignette::lerp;
use crate::display::font::draw_string;
use crate::display::fx::blend_rect;
use crate::display::gop::{fill_rect, is_initialized, vline};

const BG: u32 = 0xFF04070B;
const TRACK: u32 = 0xFF2C3B3F;
const CYAN_DIM: u32 = 0xFF0A5F58;
const CYAN_HOT: u32 = 0xFF9BFFF1;
const LABEL: u32 = 0xFF46AEB6;

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
    let lay = splash();
    let (cur, total) = get_progress();
    let pct = if total == 0 { 0 } else { (cur as u32 * 100) / total as u32 };
    let (x, y, w) = (lay.bar_x, lay.bar_y, lay.bar_w);

    fill_rect(x, y.saturating_sub(26), w, 18, BG);
    draw_string(x, y.saturating_sub(26), b"verified boot", LABEL);
    draw_pct(x + w.saturating_sub(32), y.saturating_sub(26), pct);

    fill_rect(x, y, w, 3, TRACK);
    let fill_w = w * pct / 100;
    for i in 0..fill_w {
        vline(x + i, y, 3, lerp(CYAN_DIM, CYAN_HOT, i * 256 / fill_w.max(1)));
    }
    if fill_w > 0 && fill_w < w {
        let hx = x + fill_w;
        blend_rect(hx.saturating_sub(4), y.saturating_sub(4), 10, 11, CYAN_HOT, 90);
        fill_rect(hx.saturating_sub(1), y.saturating_sub(1), 3, 5, CYAN_HOT);
    }
}

fn draw_pct(x: u32, y: u32, pct: u32) {
    let p = pct.min(100);
    let mut buf = [b' '; 4];
    let n = if p >= 100 {
        buf[0..3].copy_from_slice(b"100");
        3
    } else if p >= 10 {
        buf[0] = b'0' + (p / 10) as u8;
        buf[1] = b'0' + (p % 10) as u8;
        2
    } else {
        buf[0] = b'0' + p as u8;
        1
    };
    buf[n] = b'%';
    let s = &buf[..n + 1];
    let sx = x.saturating_sub((n as u32).saturating_sub(1) * 8);
    draw_string(sx, y, s, LABEL);
}
