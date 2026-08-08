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

use super::number::push_u64;
use alloc::string::String;

/// Bytes as a JSON array of numbers, which is how serde renders `Vec<u8>`.
/// Not base64: the gateway parses this field as a sequence of integers and a
/// string there is a type error, not a decoding difference.
pub fn push_bytes(out: &mut String, bytes: &[u8]) {
    out.push('[');
    for (i, byte) in bytes.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        push_u64(out, *byte as u64);
    }
    out.push(']');
}
