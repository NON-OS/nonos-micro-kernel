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

//! Writing the binary index.

extern crate alloc;

use alloc::vec::Vec;

use crate::sha1::Sha1;

use super::entry::IndexEntry;
use super::mode_word::mode_word;

/// Encode entries as a version 2 index, ending with the SHA-1 of everything
/// before it, which is how git detects a truncated or edited index. Entries
/// must already be sorted by path; git binary searches them.
pub fn encode(entries: &[IndexEntry]) -> Vec<u8> {
    let mut out = Vec::with_capacity(64 + entries.len() * 80);
    out.extend_from_slice(b"DIRC");
    out.extend_from_slice(&2u32.to_be_bytes());
    out.extend_from_slice(&(entries.len() as u32).to_be_bytes());

    for entry in entries {
        let start = out.len();
        // ctime, mtime, dev, ino, uid and gid: the stat cache, written as
        // zeros. Git reads that as "cannot trust, compare the content".
        out.extend_from_slice(&[0u8; 24]);
        out.extend_from_slice(&mode_word(entry).to_be_bytes());
        out.extend_from_slice(&[0u8; 8]);
        out.extend_from_slice(&entry.size.to_be_bytes());
        out.extend_from_slice(entry.id.as_bytes());

        let name = entry.path.as_bytes();
        out.extend_from_slice(&(core::cmp::min(name.len(), 0x0FFF) as u16).to_be_bytes());
        out.extend_from_slice(name);
        out.resize(out.len() + 8 - ((out.len() - start) % 8), 0);
    }

    let digest = Sha1::digest(&out);
    out.extend_from_slice(&digest);
    out
}
