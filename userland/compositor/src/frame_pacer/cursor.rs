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

const FILL: u32 = 0xFFFF_FFFF;
const OUTLINE: u32 = 0xFF00_0000;

// A classic left pointer, one byte per pixel: 'X' is the black outline, 'o' the
// white interior, a space is transparent. The tip sits at the top-left so the
// hotspot lands on the reported pointer position.
const ARROW: [&[u8]; 19] = [
    b"X",
    b"XX",
    b"XoX",
    b"XooX",
    b"XoooX",
    b"XooooX",
    b"XoooooX",
    b"XooooooX",
    b"XoooooooX",
    b"XooooooooX",
    b"XoooooooooX",
    b"XooooXXXXXX",
    b"XoooXoX",
    b"XooX XoX",
    b"XoX  XoX",
    b"XX   XoX",
    b"X     XoX",
    b"      XoX",
    b"      XXX",
];

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
    for (row, line) in ARROW.iter().enumerate() {
        for (col, &pixel) in line.iter().enumerate() {
            let argb = match pixel {
                b'X' => OUTLINE,
                b'o' => FILL,
                _ => continue,
            };
            put(cx + col as u32, cy + row as u32, argb);
        }
    }
}
