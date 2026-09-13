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

//! One entry as one line: pid, the start of the measurement, whether the mask
//! reaches the network, then every capability by name.

use alloc::vec::Vec;

use super::hexdump::push_hex;
use crate::command::builtin::cap_names::CAP_NAMES;
use crate::command::output::Output;
use crate::term::util::format_u64;

/// Taken from the name table, so the column and the list cannot disagree.
pub(super) fn network_bit() -> u64 {
    CAP_NAMES.iter().position(|n| *n == b"Network").map_or(0, |i| 1 << i)
}

pub(super) fn caps_of(e: &[u8]) -> u64 {
    u64::from_be_bytes(e[36..44].try_into().unwrap_or([0; 8]))
}

/// The kernel's authority byte: 0 vendor, 255 publisher, otherwise a
/// developer slot plus one. Padded to the column.
fn signed_by(byte: u8) -> &'static [u8] {
    match byte {
        0 => b"vendor     ",
        255 => b"publisher  ",
        _ => b"developer  ",
    }
}

pub(super) fn heading(out: &mut Output<'_>) {
    out.writeln(b"  pid  measurement   network  signed by  capabilities");
    out.writeln(b"  ---  ------------  -------  ---------  ------------");
}

pub(super) fn row(out: &mut Output<'_>, e: &[u8]) {
    let pid = u32::from_be_bytes(e[..4].try_into().unwrap_or([0; 4]));
    let caps = caps_of(e);
    let mut line: Vec<u8> = Vec::with_capacity(96);
    let mut num = [0u8; 12];
    let n = format_u64(pid as u64, &mut num);
    line.extend_from_slice(&b"      "[..6 - n.min(5)]);
    line.extend_from_slice(&num[..n]);
    line.extend_from_slice(b"  ");
    push_hex(&mut line, &e[4..10]);
    line.extend_from_slice(if caps & network_bit() != 0 { b"    YES    " } else { b"    no     " });
    line.extend_from_slice(signed_by(e[44]));
    let mut first = true;
    for (i, name) in CAP_NAMES.iter().enumerate() {
        if caps & (1u64 << i) != 0 {
            if !first {
                line.extend_from_slice(b", ");
            }
            line.extend_from_slice(name);
            first = false;
        }
    }
    out.writeln(&line);
}
