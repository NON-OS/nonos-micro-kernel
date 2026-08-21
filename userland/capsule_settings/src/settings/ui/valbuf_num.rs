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

use super::valbuf::ValBuf;

impl ValBuf {
    pub fn push_u32(&mut self, mut v: u32) {
        let mut digits = [0u8; 10];
        let mut n = 0;
        loop {
            digits[n] = b'0' + (v % 10) as u8;
            n += 1;
            v /= 10;
            if v == 0 {
                break;
            }
        }
        while n > 0 {
            n -= 1;
            self.push(digits[n]);
        }
    }

    pub fn push_i8(&mut self, v: i8) {
        if v < 0 {
            self.push(b'-');
        }
        self.push_u32(v.unsigned_abs() as u32);
    }

    pub fn push_ipv4(&mut self, ip: [u8; 4]) {
        for (i, o) in ip.iter().enumerate() {
            if i > 0 {
                self.push(b'.');
            }
            self.push_u32(*o as u32);
        }
    }
}
