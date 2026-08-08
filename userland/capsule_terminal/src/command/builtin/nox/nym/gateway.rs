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

use alloc::vec::Vec;

use crate::term::util::format_u64;

/// Render the gateway a session is bound to as `address:port`.
pub fn push_gateway(out: &mut Vec<u8>, health: &[u8]) {
    for (i, &octet) in health[..4].iter().enumerate() {
        if i > 0 {
            out.push(b'.');
        }
        push_num(out, octet as u64);
    }
    out.push(b':');
    push_num(out, u16::from_le_bytes([health[4], health[5]]) as u64);
}

fn push_num(out: &mut Vec<u8>, v: u64) {
    let mut buf = [0u8; 24];
    let k = format_u64(v, &mut buf);
    out.extend_from_slice(&buf[..k]);
}
