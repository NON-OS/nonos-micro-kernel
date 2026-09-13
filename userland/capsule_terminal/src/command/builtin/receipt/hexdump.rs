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

//! The entries as hex, one per line, for carrying off the machine.
//!
//! One entry per line so a person can count them against the summary, and so
//! a line lost in copying shows up as a ragged set rather than a quiet gap.
//! Concatenated and decoded, the lines are exactly the bytes `nonos-receipt`
//! folds.

use alloc::vec::Vec;
use core::slice::ChunksExact;

use crate::command::output::Output;

pub(super) fn hexdump(out: &mut Output<'_>, entries: ChunksExact<'_, u8>) {
    for e in entries {
        let mut line: Vec<u8> = Vec::with_capacity(e.len() * 2);
        push_hex(&mut line, e);
        out.writeln(&line);
    }
}

pub(super) fn push_hex(out: &mut Vec<u8>, bytes: &[u8]) {
    for b in bytes {
        out.push(nibble(b >> 4));
        out.push(nibble(b & 0xf));
    }
}

fn nibble(v: u8) -> u8 {
    if v < 10 {
        b'0' + v
    } else {
        b'a' + v - 10
    }
}
