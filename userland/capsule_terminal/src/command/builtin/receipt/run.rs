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

//! Reading the entries and printing them.

use alloc::vec;

use nonos_libc::{mk_attest_entries, ATTEST_ENTRY_LEN};

use super::hexdump::hexdump;
use super::row::{heading, row};
use super::summary::summary;
use crate::command::output::Output;

/// The kernel's registry holds at most 256 capsules; sized to take all of it,
/// because a short buffer is refused without saying how short.
const MAX_ENTRIES: usize = 256;

pub fn run(out: &mut Output<'_>, argv: &[&[u8]]) {
    let mut buf = vec![0u8; MAX_ENTRIES * ATTEST_ENTRY_LEN];
    let rc = mk_attest_entries(&mut buf);
    if rc < 0 {
        out.writeln(b"the kernel did not hand over its registry");
        return;
    }
    let bytes = &buf[..(rc as usize).min(buf.len())];
    if !bytes.len().is_multiple_of(ATTEST_ENTRY_LEN) {
        out.writeln(b"the kernel returned a ragged set; this build and the kernel disagree");
        return;
    }
    let entries = bytes.chunks_exact(ATTEST_ENTRY_LEN);
    if argv.iter().skip(1).any(|a| *a == b"--hex") {
        hexdump(out, entries);
        return;
    }
    heading(out);
    for e in entries.clone() {
        row(out, e);
    }
    summary(out, entries);
}
