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

pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl Rect {
    pub fn contains(&self, px: i32, py: i32) -> bool {
        px >= self.x as i32
            && px < (self.x + self.w) as i32
            && py >= self.y as i32
            && py < (self.y + self.h) as i32
    }
}

pub struct Layout {
    pub back: Rect,
    pub play: Rect,
    pub stop: Rect,
    pub fwd: Rect,
    pub progress: Rect,
    pub volume: Rect,
    pub waveform: Rect,
}

pub fn layout(width: u32, height: u32) -> Layout {
    let m = 24u32;
    let gap = 16u32;
    let cw = width.saturating_sub(m * 2);
    let bw = cw.saturating_sub(gap * 3) / 4;
    let bh = 44u32;
    let by = height.saturating_sub(bh + 26);
    let bar_h = 12u32;
    let vol_y = by.saturating_sub(28);
    let prog_y = vol_y.saturating_sub(22);
    let wf_y = 96u32;
    let wf_h = prog_y.saturating_sub(wf_y + 14);
    Layout {
        back: Rect { x: m, y: by, w: bw, h: bh },
        play: Rect { x: m + (bw + gap), y: by, w: bw, h: bh },
        stop: Rect { x: m + (bw + gap) * 2, y: by, w: bw, h: bh },
        fwd: Rect { x: m + (bw + gap) * 3, y: by, w: bw, h: bh },
        progress: Rect { x: m, y: prog_y, w: cw, h: bar_h },
        volume: Rect { x: m, y: vol_y, w: cw, h: bar_h },
        waveform: Rect { x: m, y: wf_y, w: cw, h: wf_h },
    }
}
