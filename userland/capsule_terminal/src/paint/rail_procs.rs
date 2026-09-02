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

use super::rail_fmt::{num, pct};
use super::rail_text::{clipped, head, left, lh, right, RAIL_GAP};
use crate::rail::derive::mem_pct;
use crate::rail::metrics::Sample;
use crate::term::theme::types::Theme;

pub use super::rail_geom::procs_h as height;

const PID_W: u32 = 40;
const CPU_W: u32 = 46;
const MEM_W: u32 = 46;

/// The live set, hottest first. Numeric columns are right-aligned off measured
/// edges and the name is cut by width, so a long capsule name can never shove a
/// percentage off the rail.
pub fn draw(fb: &mut PaintBuffer, x: u32, y: i32, w: u32, s: &Sample, t: &Theme) {
    let row = lh() as i32;
    let cpu_edge = x + w.saturating_sub(MEM_W);
    let mem_edge = x + w;
    let name_w = w.saturating_sub(PID_W + CPU_W + MEM_W + RAIL_GAP);
    let mut y = head(fb, x, y, w, "PROCESSES", t);
    left(fb, x, y, "PID", t.dim);
    left(fb, x + PID_W, y, "NAME", t.dim);
    right(fb, cpu_edge, y, "CPU", t.dim);
    right(fb, mem_edge, y, "MEM", t.dim);
    y += row + (RAIL_GAP / 2) as i32;
    let mut buf = [0u8; 24];
    for p in s.live() {
        left(fb, x, y, num(&mut buf, p.pid as u64), t.dim);
        clipped(fb, x + PID_W, y, name_w, p.name_str(), t.fg);
        let hot = if p.cpu_pct >= 50 { t.warn } else { t.fg };
        right(fb, cpu_edge, y, pct(&mut buf, p.cpu_pct), hot);
        right(fb, mem_edge, y, pct(&mut buf, mem_pct(p.mem_kb, s.mem_total_kb)), t.dim);
        y += row;
    }
}
