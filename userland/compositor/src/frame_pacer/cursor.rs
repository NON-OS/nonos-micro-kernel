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

use crate::state::damage::Rect;

const FG: u32 = 0xFFFF_FFFF;
const SHADOW: u32 = 0xFF00_0000;
const SIZE: u32 = 14;
const THICK: u32 = 2;

pub fn blit(base_va: u64, stride: u32, sw: u32, sh: u32, cx: u32, cy: u32, clip: Rect) {
    let clip_x1 = clip.x + clip.width;
    let clip_y1 = clip.y + clip.height;
    let put = |px: u32, py: u32, argb: u32| {
        if px >= sw || py >= sh || px < clip.x || px >= clip_x1 || py < clip.y || py >= clip_y1 {
            return;
        }
        let va = base_va as usize + py as usize * stride as usize + px as usize * 4;
        unsafe { core::ptr::write_volatile(va as *mut u32, argb) };
    };
    for i in 0..SIZE {
        for t in 0..THICK {
            put(cx + 1 + t, cy + i, SHADOW);
            put(cx + i, cy + 1 + t, SHADOW);
        }
    }
    for i in 0..SIZE {
        for t in 0..THICK {
            put(cx + t, cy + i, FG);
            put(cx + i, cy + t, FG);
        }
    }
}
