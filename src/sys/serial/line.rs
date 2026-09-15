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

//! A whole line, assembled before the port is touched.
//!
//! `print`, `print_dec` and `print_hex` each take the serial lock for the
//! duration of that call, so a line built from several of them is several
//! critical sections with gaps between. On one CPU nothing can run in the
//! gaps. On four, two CPUs' lines interleave character by character:
//!
//!   [SMP-PROOF] cpu_count=[APIC] Setting up timer at 1100 UP
//!
//! which is `cpu_count=1 UP` with another CPU's line spliced through it. That
//! is not only unreadable, it is unparseable, and the boot matrix reads these
//! lines to decide whether a cell passed.
//!
//! Holding the lock across the whole line instead would be the obvious fix and
//! would deadlock: the lock is a plain spin mutex and any `print` reached from
//! inside the held region would take it again. So the line is built in a
//! caller-owned buffer and handed to the port in one call.

use super::print::println;

/// Bytes a single diagnostic line may carry. Longer lines are truncated
/// rather than split, because half a line that looks whole is what this
/// module exists to prevent.
const LINE_MAX: usize = 192;

pub struct Line {
    buf: [u8; LINE_MAX],
    len: usize,
}

impl Default for Line {
    fn default() -> Self {
        Self::new()
    }
}

impl Line {
    pub const fn new() -> Self {
        Self { buf: [0; LINE_MAX], len: 0 }
    }

    fn push(&mut self, b: u8) {
        if self.len < LINE_MAX {
            self.buf[self.len] = b;
            self.len += 1;
        }
    }

    pub fn str(&mut self, s: &[u8]) -> &mut Self {
        for &b in s {
            self.push(b);
        }
        self
    }

    pub fn dec(&mut self, mut val: u64) -> &mut Self {
        if val == 0 {
            self.push(b'0');
            return self;
        }
        let mut digits = [0u8; 20];
        let mut n = 0;
        while val > 0 {
            digits[n] = b'0' + (val % 10) as u8;
            val /= 10;
            n += 1;
        }
        while n > 0 {
            n -= 1;
            self.push(digits[n]);
        }
        self
    }

    pub fn hex(&mut self, val: u64) -> &mut Self {
        const HEX: &[u8] = b"0123456789ABCDEF";
        self.str(b"0x");
        for i in (0..16).rev() {
            self.push(HEX[((val >> (i * 4)) & 0xF) as usize]);
        }
        self
    }

    /// Emit the line, one lock acquisition, one newline.
    pub fn end(&self) {
        println(&self.buf[..self.len]);
    }
}
