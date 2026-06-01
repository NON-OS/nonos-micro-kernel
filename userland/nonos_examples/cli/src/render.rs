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

use super::hex::hex_digit;
use nonos_runtime::log;

pub fn render_line(boot_ms: i64, seed: &[u8]) {
    let mut line = [0u8; 96];
    let mut n = 0;
    for &b in b"boot_ms=0x" {
        line[n] = b;
        n += 1;
    }
    for shift in (0..16).rev() {
        line[n] = hex_digit(((boot_ms as u64 >> (shift * 4)) & 0xF) as u8);
        n += 1;
    }
    for &b in b" seed=0x" {
        line[n] = b;
        n += 1;
    }
    for &byte in seed {
        line[n] = hex_digit(byte >> 4);
        n += 1;
        line[n] = hex_digit(byte);
        n += 1;
    }
    line[n] = b'\n';
    n += 1;
    let _ = log::log(&line[..n]);
}
