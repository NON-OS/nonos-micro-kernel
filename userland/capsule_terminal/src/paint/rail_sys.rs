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

use super::rail_fmt::{mib_into, pct, u32_into, uptime};
use super::rail_geom::SPARK_H;
use super::rail_metric::{one, pair};
use super::rail_stat::stat;
use super::rail_text::{bar, head, BAR_H, RAIL_GAP};
use super::spark::draw_spark;
use crate::layout::Rect;
use crate::rail::Rail;
use crate::term::theme::types::Theme;

pub use super::rail_geom::sys_h as height;

/// What the machine is doing now and what it has been doing. Load and swap have
/// no source on NONOS and say so; the CPU figure is real and carries both the
/// bar and the trend beneath it.
pub fn draw(fb: &mut PaintBuffer, x: u32, y: i32, w: u32, rail: &Rail, t: &Theme) {
    let s = &rail.sample;
    let mut b = [0u8; 48];
    let mut y = head(fb, x, y, w, "SYSTEM", t);
    y = stat(fb, x, y, w, "UPTIME", uptime(&mut b, s.uptime_ms), t);
    y = stat(fb, x, y, w, "LOAD", one(&mut b, s.load_avg, u32_into), t);
    y = stat(fb, x, y, w, "CPU", pct(&mut b, s.cpu_pct), t);
    bar(fb, x, y, w, s.cpu_pct, t);
    y += (BAR_H + RAIL_GAP) as i32;
    if y >= 0 {
        let r = Rect { x, y: y as u32, w, h: SPARK_H };
        draw_spark(fb, r, rail.spark.slice(), rail.spark.start(), t.accent);
    }
    y += (SPARK_H + RAIL_GAP) as i32;
    y = stat(fb, x, y, w, "MEMORY", pair(&mut b, s.mem.used_kb, s.mem.total_kb, mib_into), t);
    stat(fb, x, y, w, "SWAP", one(&mut b, s.mem.swap_used_kb, mib_into), t);
}
