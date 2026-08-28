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

pub const CAP: usize = 24;

// A number rendered without an allocator. The digits are laid down from the
// end of the buffer, so the slice the painters read is always the tail.
pub struct Digits {
    buf: [u8; CAP],
    len: usize,
}

impl Digits {
    pub fn as_bytes(&self) -> &[u8] {
        &self.buf[CAP - self.len..]
    }
}

pub fn tail(buf: [u8; CAP], at: usize) -> Digits {
    Digits { buf, len: CAP - at }
}

pub fn dec(mut value: u32) -> Digits {
    let mut buf = [b'0'; CAP];
    let mut at = CAP;
    loop {
        at -= 1;
        buf[at] = b'0' + (value % 10) as u8;
        value /= 10;
        if value == 0 {
            break;
        }
    }
    tail(buf, at)
}
