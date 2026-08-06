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
//! Framing the command list and the pack behind it.

extern crate alloc;

use alloc::vec::Vec;

use super::super::pkt::encode_pkt;
use super::command::RefUpdate;

/// Build the request body: one line per ref, a flush, then the pack.
///
/// The capability list rides on the first command line after a NUL, which is
/// where the protocol puts it. `report-status` is what makes the receiver
/// answer with whether the pack unpacked and each ref moved, instead of
/// closing the connection silently.
pub fn push_request(updates: &[RefUpdate<'_>], pack: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    for (i, update) in updates.iter().enumerate() {
        let mut line = Vec::new();
        line.extend_from_slice(update.old.to_hex().as_bytes());
        line.push(b' ');
        line.extend_from_slice(update.new.to_hex().as_bytes());
        line.push(b' ');
        line.extend_from_slice(update.name.as_bytes());
        if i == 0 {
            line.push(0);
            line.extend_from_slice(b"report-status");
        }
        line.push(b'\n');
        encode_pkt(&line, &mut out);
    }
    out.extend_from_slice(b"0000");
    out.extend_from_slice(pack);
    out
}
