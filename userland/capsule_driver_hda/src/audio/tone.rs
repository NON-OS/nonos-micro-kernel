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

use core::ptr::write_volatile;

const HALF_PERIOD: usize = 55;
const AMPLITUDE: i16 = 0x2000;

pub fn fill(va: u64, bytes: usize) {
    let frames = bytes / 4;
    let buf = va as *mut i16;
    let mut i = 0usize;
    while i < frames {
        let sample = if (i / HALF_PERIOD) & 1 == 0 { AMPLITUDE } else { -AMPLITUDE };
        unsafe {
            write_volatile(buf.add(i * 2), sample);
            write_volatile(buf.add(i * 2 + 1), sample);
        }
        i += 1;
    }
}
