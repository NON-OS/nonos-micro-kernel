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

#![no_std]
#![no_main]

extern crate alloc;

use nonos_libc::{heap_init_sized, mk_exit};
use zune_jpeg::JpegDecoder;

const PROBE_HEAP_BYTES: usize = 1 << 20;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init_sized(PROBE_HEAP_BYTES).is_err() {
        mk_exit(1);
    }
    let mut decoder = JpegDecoder::new(&[] as &[u8]);
    let status = match decoder.decode_headers() {
        Ok(()) => 0,
        Err(_) => 2,
    };
    mk_exit(status)
}
