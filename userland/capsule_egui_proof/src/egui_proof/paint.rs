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

use super::frame::Stats;
use super::manifest::WIDTH;

const BG: u32 = 0xFF10161C;
const ACCENT: u32 = 0xFF00D4AA;
const TEXT: u32 = 0xFFE8F0F8;
const DIM: u32 = 0xFF667788;
const FAIL: u32 = 0xFFFF5566;

pub fn paint(fb: &mut PaintBuffer, frames: u32, stats: Stats) {
    fb.clear(BG);
    fb.fill_rect(0, 0, WIDTH, 4, ACCENT);
    fb.text_ttf(20, 30, "egui 0.35 tessellating at CPL=3", ACCENT, 18.0);

    let rows = [
        format!("shapes    {}", stats.shapes),
        format!("meshes    {}", stats.prims),
        format!("vertices  {}", stats.verts),
        format!("indices   {}", stats.idxs),
        format!("textures  {}", stats.textures),
        format!("atlas     {}x{}", stats.atlas_w, stats.atlas_h),
        format!("frames    {frames}"),
    ];
    let mut y = 70;
    for row in &rows {
        fb.text_ttf_mono(24, y, row, TEXT, 15.0);
        y += 22;
    }

    let ok = stats.verts > 0 && stats.idxs > 0;
    let (verdict, colour) = if ok {
        ("EGUI PROOF PASS", ACCENT)
    } else {
        ("EGUI PROOF FAIL", FAIL)
    };
    fb.text_ttf(24, y + 8, verdict, colour, 20.0);
    fb.text_ttf(24, y + 38, "click to drive another frame", DIM, 13.0);
}
