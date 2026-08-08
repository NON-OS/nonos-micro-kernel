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

extern crate alloc;
use alloc::vec;

use nonos_libc::{heap_init, mk_debug, mk_exit, HeapError};

use crate::viewer::flip::flip_h;
use crate::viewer::rotate::rotate_cw;
use crate::viewer::scale::{draw_bilinear, draw_nn, Dst};
use crate::viewer::viewport::{place_mode, FitMode, View};

const EXPECTED_CHECKSUM: u64 = 301_987_575;

pub fn run() -> ! {
    match heap_init() {
        Ok(()) | Err(HeapError::AlreadyInitialized) => {}
        Err(_) => exit_fail(b"[IMG-VIEWER] FAIL heap\n"),
    }

    let src: [u32; 4] = [0xFF0000FF, 0xFF00FF00, 0xFFFF0000, 0xFFFFFF00];
    let (rotated, rw, rh) = rotate_cw(&src, 2, 2);

    let mut dst_px = vec![0u32; 16];
    let view = View { zoom: 1.0, pan_x: 0.0, pan_y: 0.0 };
    let p = place_mode(FitMode::Fit, rw, rh, 4, 4, &view);
    {
        let mut dst = Dst { px: &mut dst_px, stride: 4, w: 4, h: 4 };
        draw_nn(&mut dst, &rotated, rw, rh, p.dx, p.dy, p.dw, p.dh);
    }
    let mut checksum: u64 = dst_px.iter().map(|&px| (px & 0x00FF_FFFF) as u64).sum();

    let flipped = flip_h(&src, 2, 2);
    checksum += flipped.iter().map(|&px| (px & 0x00FF_FFFF) as u64).sum::<u64>();

    let mut bi_px = vec![0u32; 16];
    let bp = place_mode(FitMode::Fit, 2, 2, 4, 4, &view);
    {
        let mut bdst = Dst { px: &mut bi_px, stride: 4, w: 4, h: 4 };
        draw_bilinear(&mut bdst, &src, 2, 2, bp.dx, bp.dy, bp.dw, bp.dh);
    }
    checksum += bi_px.iter().map(|&px| (px & 0x00FF_FFFF) as u64).sum::<u64>();

    let big_w = 200u32;
    let big_h = 150u32;
    let big_src = vec![0xFF3366CCu32; (big_w * big_h) as usize];
    let mut big_dst = vec![0u32; (big_w * big_h) as usize];
    let bview = View { zoom: 1.0, pan_x: 0.0, pan_y: 0.0 };
    let bp2 = place_mode(FitMode::Fit, big_w, big_h, big_w, big_h, &bview);
    {
        let mut bd = Dst { px: &mut big_dst, stride: big_w, w: big_w, h: big_h };
        draw_bilinear(&mut bd, &big_src, big_w, big_h, bp2.dx, bp2.dy, bp2.dw, bp2.dh);
    }
    if big_dst[(big_w * big_h / 2) as usize] & 0x00FF_FFFF != 0x3366CC {
        exit_fail(b"[IMG-VIEWER] FAIL bigscale\n");
    }

    let gg = crate::viewer::gallery::layout::grid(800);
    if gg.cols == 0 {
        exit_fail(b"[IMG-VIEWER] FAIL gridcols\n");
    }
    let (gx, _gy, gw, _gh) = crate::viewer::gallery::layout::cell_rect(0, 0, &gg);
    if gx < 0 || gw == 0 {
        exit_fail(b"[IMG-VIEWER] FAIL cellrect\n");
    }

    if checksum == EXPECTED_CHECKSUM {
        emit(b"[IMG-VIEWER] PASS\n");
        mk_exit(0);
    } else {
        exit_fail(b"[IMG-VIEWER] FAIL checksum\n");
    }
}

fn emit(msg: &[u8]) {
    let _ = mk_debug(msg.as_ptr(), msg.len());
}

fn exit_fail(msg: &[u8]) -> ! {
    emit(msg);
    mk_exit(1);
}
