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
//! The type and size that precede an object's zlib stream.

extern crate alloc;

use alloc::vec::Vec;

use crate::object::ObjectKind;

/// Append the entry header for an object of `kind` and `size` bytes.
///
/// The first byte carries the type in bits 4 to 6 and the low four bits of the
/// size; every further byte carries seven more size bits, low group first, with
/// the top bit set while more follow.
pub(super) fn write(kind: ObjectKind, size: usize, out: &mut Vec<u8>) {
    let type_bits = match kind {
        ObjectKind::Commit => 1u8,
        ObjectKind::Tree => 2,
        ObjectKind::Blob => 3,
        ObjectKind::Tag => 4,
    };
    let mut left = size;
    let mut byte = (type_bits << 4) | (left as u8 & 0x0F);
    left >>= 4;
    while left > 0 {
        out.push(byte | 0x80);
        byte = left as u8 & 0x7F;
        left >>= 7;
    }
    out.push(byte);
}
