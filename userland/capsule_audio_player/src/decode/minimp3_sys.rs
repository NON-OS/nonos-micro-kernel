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

use core::ffi::c_int;

pub const MINIMP3_MAX_SAMPLES_PER_FRAME: usize = 2 * 1152;

const MP3DEC_T_SIZE: usize = 6668;

#[repr(C)]
pub struct mp3dec_frame_info_t {
    pub frame_bytes: c_int,
    pub frame_offset: c_int,
    pub channels: c_int,
    pub hz: c_int,
    pub layer: c_int,
    pub bitrate_kbps: c_int,
}

#[repr(C, align(4))]
pub struct mp3dec_t {
    _priv: [u8; MP3DEC_T_SIZE],
}

pub type Mp3decInit = unsafe extern "C" fn(dec: *mut mp3dec_t);
pub type Mp3decDecodeFrame = unsafe extern "C" fn(
    dec: *mut mp3dec_t,
    mp3: *const u8,
    mp3_bytes: c_int,
    pcm: *mut i16,
    info: *mut mp3dec_frame_info_t,
) -> c_int;

extern "C" {
    pub fn mp3dec_init(dec: *mut mp3dec_t);
    pub fn mp3dec_decode_frame(
        dec: *mut mp3dec_t,
        mp3: *const u8,
        mp3_bytes: c_int,
        pcm: *mut i16,
        info: *mut mp3dec_frame_info_t,
    ) -> c_int;
}

#[used]
static _MINIMP3_KEEP: (Mp3decInit, Mp3decDecodeFrame) = (mp3dec_init, mp3dec_decode_frame);
