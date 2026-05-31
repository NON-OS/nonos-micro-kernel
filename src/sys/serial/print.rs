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

use super::core::{with_serial_lock, write_byte};

fn emit(s: &[u8]) {
    for &ch in s {
        write_byte(ch);
    }
}

pub fn print(s: &[u8]) {
    with_serial_lock(|| emit(s));
}

pub fn print_str(s: &str) {
    print(s.as_bytes());
}

pub fn println(s: &[u8]) {
    with_serial_lock(|| {
        emit(s);
        emit(b"\r\n");
    });
}

pub fn print_hex(val: u64) {
    const HEX: &[u8] = b"0123456789ABCDEF";
    with_serial_lock(|| {
        emit(b"0x");
        for i in (0..16).rev() {
            let nibble = ((val >> (i * 4)) & 0xF) as usize;
            write_byte(HEX[nibble]);
        }
    });
}

pub fn print_dec(mut val: u64) {
    with_serial_lock(|| {
        if val == 0 {
            write_byte(b'0');
            return;
        }
        let mut buf = [0u8; 20];
        let mut i = 0;
        while val > 0 {
            buf[i] = b'0' + (val % 10) as u8;
            val /= 10;
            i += 1;
        }
        while i > 0 {
            i -= 1;
            write_byte(buf[i]);
        }
    });
}
