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
use alloc::boxed::Box;
use core::ffi::c_int;
use super::minimp3_sys::{
    mp3dec_decode_frame, mp3dec_frame_info_t, mp3dec_init, mp3dec_t,
    MINIMP3_MAX_SAMPLES_PER_FRAME,
};

pub const MAX_SAMPLES: usize = MINIMP3_MAX_SAMPLES_PER_FRAME;

pub fn new_decoder() -> Box<mp3dec_t> {
    let mut dec: Box<mp3dec_t> = unsafe { Box::<mp3dec_t>::new_zeroed().assume_init() };
    unsafe {
        mp3dec_init(&mut *dec);
    }
    dec
}

pub fn empty_frame_info() -> mp3dec_frame_info_t {
    mp3dec_frame_info_t {
        frame_bytes: 0,
        frame_offset: 0,
        channels: 0,
        hz: 0,
        layer: 0,
        bitrate_kbps: 0,
    }
}

pub fn decode_frame(
    dec: &mut mp3dec_t,
    input: &[u8],
    pcm: &mut [i16; MAX_SAMPLES],
    fi: &mut mp3dec_frame_info_t,
) -> c_int {
    unsafe {
        mp3dec_decode_frame(dec, input.as_ptr(), input.len() as c_int, pcm.as_mut_ptr(), fi)
    }
}
