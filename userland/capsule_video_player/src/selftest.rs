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

use nonos_avi::AviFile;
use nonos_libc::{heap_init_sized, mk_debug, mk_exit, HeapError};

use crate::player::{column_map, letterbox, scale_into, FrameDecoder};

const CLIP: &[u8] = include_bytes!("../assets/clip.avi");
const SELFTEST_HEAP: usize = 32 * 1024 * 1024;
const FRAMES: usize = 12;
const DST_W: u32 = 320;
const DST_H: u32 = 240;

pub fn run() -> ! {
    match heap_init_sized(SELFTEST_HEAP) {
        Ok(()) | Err(HeapError::AlreadyInitialized) => {}
        Err(_) => fail(b"[VIDEO] FAIL heap\n"),
    }
    let Ok(file) = AviFile::parse(CLIP) else {
        fail(b"[VIDEO] FAIL parse\n")
    };
    let (w, h) = (file.video.width, file.video.height);
    if w != 160 || h != 120 {
        fail(b"[VIDEO] FAIL dims\n");
    }
    if file.index.len() != FRAMES {
        fail(b"[VIDEO] FAIL index\n");
    }
    let mut decoder = FrameDecoder::new();
    let mut frame = vec![0u32; (w * h) as usize];
    for fr in file.index.iter() {
        let Some(end) = fr.offset.checked_add(fr.len as u64) else {
            fail(b"[VIDEO] FAIL span\n")
        };
        let Some(bytes) = CLIP.get(fr.offset as usize..end as usize) else {
            fail(b"[VIDEO] FAIL bounds\n")
        };
        if decoder.decode(bytes, w, h, &mut frame).is_err() {
            fail(b"[VIDEO] FAIL decode\n");
        }
    }
    let rect = letterbox(w, h, DST_W, DST_H);
    let cols = column_map(w, rect.2);
    let mut dst = vec![0u32; (DST_W * DST_H) as usize];
    scale_into(&frame, w, h, &mut dst, DST_W, rect, &cols);
    let centre = ((rect.1 + rect.3 / 2) * DST_W + rect.0 + rect.2 / 2) as usize;
    if dst.get(centre).map(|p| p >> 24) != Some(0xff) {
        fail(b"[VIDEO] FAIL scale\n");
    }
    emit(b"[VIDEO] PASS\n");
    mk_exit(0)
}

fn emit(msg: &[u8]) {
    let _ = mk_debug(msg.as_ptr(), msg.len());
}

fn fail(msg: &[u8]) -> ! {
    emit(msg);
    mk_exit(1)
}
