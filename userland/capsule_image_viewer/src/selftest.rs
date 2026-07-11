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

use crate::viewer::rotate::rotate_cw;
use crate::viewer::scale::{draw_nn, Dst};
use crate::viewer::viewport::{place, View};

const EXPECTED_CHECKSUM: u64 = 134_216_700;

pub fn run() -> ! {
    match heap_init() {
        Ok(()) | Err(HeapError::AlreadyInitialized) => {}
        Err(_) => exit_fail(b"[IMG-VIEWER] FAIL heap\n"),
    }

    let src: [u32; 4] = [0xFF0000FF, 0xFF00FF00, 0xFFFF0000, 0xFFFFFF00];
    let (rotated, rw, rh) = rotate_cw(&src, 2, 2);

    let mut dst_px = vec![0u32; 16];
    let view = View { zoom: 1.0, pan_x: 0.0, pan_y: 0.0 };
    let p = place(rw, rh, 4, 4, &view);
    {
        let mut dst = Dst { px: &mut dst_px, stride: 4, w: 4, h: 4 };
        draw_nn(&mut dst, &rotated, rw, rh, p.dx, p.dy, p.dw, p.dh);
    }

    let checksum: u64 = dst_px.iter().map(|&px| (px & 0x00FF_FFFF) as u64).sum();
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
