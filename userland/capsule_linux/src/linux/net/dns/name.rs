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

//! The name out of a DNS question.

use alloc::vec::Vec;

/// Where the question starts: past the twelve byte header.
pub const QUESTION: usize = 12;

/// The longest name that can be asked for, as the format allows.
const MAX_NAME: usize = 255;

/// The name, and the offset of the byte after it.
pub fn read(msg: &[u8]) -> Option<(Vec<u8>, usize)> {
    let mut out: Vec<u8> = Vec::new();
    let mut at = QUESTION;
    loop {
        let len = *msg.get(at)? as usize;
        if len == 0 {
            return Some((out, at + 1));
        }
        if len > 63 || out.len() + len + 1 > MAX_NAME {
            return None;
        }
        let from = at + 1;
        let label = msg.get(from..from + len)?;
        if !out.is_empty() {
            out.push(b'.');
        }
        out.extend_from_slice(label);
        at = from + len;
    }
}
