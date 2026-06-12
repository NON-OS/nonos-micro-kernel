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

use super::format::u32_decimal;
use super::manifest::WIDTH;
use super::state::{CpuSample, State, HISTORY};
use super::theme::{BACKGROUND, FOREGROUND, MUTED, WARNING};

const TEXT_LEFT: u32 = 16;
const SPARK_X: u32 = WIDTH - 30 - 56;
const SPARK_H: u32 = 12;
const PCT_X: u32 = WIDTH - 48;

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    fb.clear(BACKGROUND);
    fb.text(TEXT_LEFT, 18, b"process_manager", FOREGROUND);
    fb.text(TEXT_LEFT, 40, b"name", WARNING);
    fb.text(TEXT_LEFT + 140, 40, b"pid", WARNING);
    fb.text(TEXT_LEFT + 220, 40, b"caps", WARNING);
    fb.text(TEXT_LEFT, 212, state.status, MUTED);
    let mut digits = [0u8; 10];
    let n = u32_decimal(state.refreshes, &mut digits);
    fb.text(TEXT_LEFT, 232, b"refreshes:", MUTED);
    fb.text(TEXT_LEFT + 96, 232, &digits[..n], MUTED);
    let mut y = 68;
    for row in state.rows.iter() {
        fb.text(TEXT_LEFT, y, row.label, FOREGROUND);
        if row.online {
            let n = u32_decimal(row.pid, &mut digits);
            fb.text(TEXT_LEFT + 140, y, &digits[..n], FOREGROUND);
            fb.text(TEXT_LEFT + 220, y, b"unavailable", MUTED);
        } else {
            fb.text(TEXT_LEFT + 140, y, b"offline", MUTED);
            fb.text(TEXT_LEFT + 220, y, b"unavailable", MUTED);
        }
        paint_spark(fb, y, &row.cpu);
        let newest = row.cpu.percent[(row.cpu.head + HISTORY - 1) % HISTORY];
        let n = u32_decimal(newest as u32, &mut digits);
        fb.text(PCT_X, y, &digits[..n], FOREGROUND);
        y += 18;
    }
}

fn paint_spark(fb: &mut PaintBuffer, y: u32, cpu: &CpuSample) {
    let base = y + SPARK_H;
    for j in 0..HISTORY {
        let pct = cpu.percent[(cpu.head + j) % HISTORY] as u32;
        let h = (pct * SPARK_H / 100).max(1);
        let color = if pct == 0 { MUTED } else { FOREGROUND };
        fb.fill_rect(SPARK_X + j as u32, base.saturating_sub(h), 1, h, color);
    }
}
