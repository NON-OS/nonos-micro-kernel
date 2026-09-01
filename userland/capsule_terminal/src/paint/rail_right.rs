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

use super::rail_fmt::{mib, pct, uptime};
use super::rail_procs;
use super::rail_text::{bar, head, left, lh, right, RAIL_GAP, RAIL_PAD};
use super::spark::draw_spark;
use crate::layout::Rect;
use crate::rail::Rail;
use crate::term::theme::types::Theme;

const SPARK_H: u32 = 44;

/// The telemetry rail: what the machine is doing, what it has been doing, and
/// who is doing it. Disk and network are absent because the kernel publishes no
/// counters for either; an empty labelled section would read as zero traffic.
pub fn draw(fb: &mut PaintBuffer, r: Rect, rail: &Rail, t: &Theme) {
    fb.blend_rect(r.x, r.y, 1, r.h, t.chrome_edge);
    let x = r.x + RAIL_PAD;
    let w = r.w.saturating_sub(RAIL_PAD * 2);
    let row = lh();
    let mut y = head(fb, x, r.y + RAIL_GAP, w, "SYSTEM", t);
    let mut buf = [0u8; 32];
    y = stat(fb, x, y, w, "UPTIME", uptime(&mut buf, rail.sample.uptime_ms), t);
    y = stat(fb, x, y, w, "CPU", pct(&mut buf, rail.sample.cpu_pct), t);
    bar(fb, x, y, w, rail.sample.cpu_pct, t);
    y += RAIL_GAP;
    y = stat(fb, x, y, w, "MEMORY", mib(&mut buf, rail.sample.mem_total_kb), t);
    y = head(fb, x, y + RAIL_GAP, w, "CPU", t);
    let spark_h = SPARK_H.min((r.y + r.h).saturating_sub(y));
    draw_spark(fb, Rect { x, y, w, h: spark_h }, rail.spark.slice(), rail.spark.start(), t.accent);
    y += spark_h + RAIL_GAP;
    y = head(fb, x, y, w, "PROCESSES", t);
    let h = (r.y + r.h).saturating_sub(y + RAIL_PAD);
    if h >= row {
        rail_procs::draw(fb, Rect { x, y, w, h }, &rail.sample, t);
    }
}

fn stat(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, k: &str, v: &str, t: &Theme) -> u32 {
    left(fb, x, y, k, t.dim);
    right(fb, x + w, y, v, t.fg);
    y + lh()
}
