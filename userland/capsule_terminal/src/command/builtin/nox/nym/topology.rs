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

/// Render the topology status body: a state word, then the directory epoch
/// and its validity window. The names match what the capsule reports so a
/// reading here can be compared against the serial log without translation.
pub fn topology_line(body: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(b"topology: ");
    out.extend_from_slice(state_name(u32::from_le_bytes([body[0], body[1], body[2], body[3]])));
    let epoch = u64::from_le_bytes([
        body[4], body[5], body[6], body[7], body[8], body[9], body[10], body[11],
    ]);
    if epoch != 0 {
        out.extend_from_slice(b" epoch ");
        let mut buf = [0u8; 24];
        let k = format_u64(epoch, &mut buf);
        out.extend_from_slice(&buf[..k]);
    }
    out
}

fn state_name(code: u32) -> &'static [u8] {
    match code {
        0 => b"missing",
        1 => b"ready",
        2 => b"expired",
        3 => b"clock out of range",
        4 => b"untrusted authority",
        _ => b"unknown",
    }
}
