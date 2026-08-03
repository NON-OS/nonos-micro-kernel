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
//! The body of a `git-upload-pack` request.

extern crate alloc;

use alloc::vec::Vec;

use crate::oid::ObjectId;

use super::pkt::encode_pkt;

/// Ask for `wants` with no history, which is what a depth-one clone sends.
///
/// The first want carries the capabilities. `no-progress` keeps the sideband
/// free of status text, and `ofs-delta` lets the server use the compact delta
/// form, which the pack reader resolves. Nothing else is claimed, so the
/// server sends a plain pack rather than a multiplexed stream.
pub fn want_request(wants: &[ObjectId], depth: u32) -> Vec<u8> {
    let mut out = Vec::new();
    for (i, id) in wants.iter().enumerate() {
        let mut line = Vec::new();
        line.extend_from_slice(b"want ");
        line.extend_from_slice(id.to_hex().as_bytes());
        if i == 0 {
            line.extend_from_slice(b" no-progress ofs-delta");
        }
        line.push(b'\n');
        encode_pkt(&line, &mut out);
    }
    if depth > 0 {
        let mut line = Vec::new();
        line.extend_from_slice(b"deepen ");
        push_decimal(&mut line, depth);
        line.push(b'\n');
        encode_pkt(&line, &mut out);
    }
    // Flush ends the want section, then `done` says there is nothing we have.
    out.extend_from_slice(b"0000");
    encode_pkt(b"done\n", &mut out);
    out
}

fn push_decimal(out: &mut Vec<u8>, mut v: u32) {
    if v == 0 {
        out.push(b'0');
        return;
    }
    let start = out.len();
    while v > 0 {
        out.push(b'0' + (v % 10) as u8);
        v /= 10;
    }
    out[start..].reverse();
}
