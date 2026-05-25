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

use nonos_libc::mk_debug;

const TAG: &[u8] = b"[INPUT-PROOF] ";
const CAP: usize = 96;

pub struct Line {
    buf: [u8; CAP],
    len: usize,
}

impl Line {
    pub fn new() -> Self {
        let mut line = Line { buf: [0u8; CAP], len: 0 };
        line.push(TAG);
        line
    }

    pub fn push(&mut self, src: &[u8]) -> &mut Self {
        for &b in src {
            if self.len < CAP {
                self.buf[self.len] = b;
                self.len += 1;
            }
        }
        self
    }

    pub fn num(&mut self, value: i32) -> &mut Self {
        let mut digits = [0u8; 11];
        let mut i = digits.len();
        let negative = value < 0;
        let mut acc = (value as i64).unsigned_abs();
        loop {
            i -= 1;
            digits[i] = b'0' + (acc % 10) as u8;
            acc /= 10;
            if acc == 0 {
                break;
            }
        }
        if negative {
            self.push(b"-");
        }
        self.push(&digits[i..])
    }

    pub fn emit(&self) {
        let _ = mk_debug(self.buf.as_ptr(), self.len);
    }
}
