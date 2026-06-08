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

use nonos_libc::AttestStatus;
use nonos_toolkit::font::render::draw_text;

const FG: u32 = 0xFFE8_F0F8;
const DIM: u32 = 0xFF5B_6B78;
const OK: u32 = 0xFF00_CC66;
const WARN: u32 = 0xFFFF_AA00;

fn hex32(src: &[u8; 32], out: &mut [u8; 64]) {
    const H: &[u8; 16] = b"0123456789abcdef";
    for (i, &b) in src.iter().enumerate() {
        out[i * 2] = H[(b >> 4) as usize];
        out[i * 2 + 1] = H[(b & 0xf) as usize];
    }
}

pub(crate) fn detail(base: u64, w: u32, h: u32, stride: u32, att: &AttestStatus) {
    let spx = stride as usize / 4;
    let buf = unsafe { core::slice::from_raw_parts_mut(base as *mut u32, spx * h as usize) };
    crate::vignette::fill(buf, spx, w, h);
    let (px, py, pw) = (40u32, 60u32, w.saturating_sub(80));
    crate::chrome::panel(buf, spx, w, h, px, py, pw, 220, b"boot-chain attestation");
    let mut hx = [0u8; 64];
    hex32(&att.kernel_blake3, &mut hx);
    draw_text(buf, spx, w, h, px + 10, py + 40, b"kernel blake3", DIM);
    draw_text(buf, spx, w, h, px + 10, py + 58, &hx, FG);
    hex32(&att.program_hash, &mut hx);
    draw_text(buf, spx, w, h, px + 10, py + 96, b"zk program hash", DIM);
    draw_text(buf, spx, w, h, px + 10, py + 114, &hx, FG);
    flag(buf, spx, w, h, px + 10, py + 152, b"secure_boot", att.secure_boot == 1);
    flag(buf, spx, w, h, px + 200, py + 152, b"attested", att.zk_verified == 1);
    draw_text(buf, spx, w, h, px + 10, h - 40, b"press any key to return", DIM);
}

fn flag(buf: &mut [u32], spx: usize, w: u32, h: u32, x: u32, y: u32, label: &[u8], ok: bool) {
    draw_text(buf, spx, w, h, x, y, label, if ok { OK } else { WARN });
}
