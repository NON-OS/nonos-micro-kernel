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

//! Walking a tar, which is what a package is once it is decompressed.

use alloc::vec::Vec;

use super::tar_field::{name_of, octal};

const BLOCK: usize = 512;
const SIZE_AT: usize = 124;
const SIZE_LEN: usize = 12;
const TYPE_AT: usize = 156;

pub struct Entry {
    pub name: Vec<u8>,
    pub body: Vec<u8>,
}

/// Regular files only. A package's directories are implied by its paths
/// and its links are followed at install time, not recreated.
pub fn entries(data: &[u8]) -> Vec<Entry> {
    let mut out = Vec::new();
    let mut at = 0usize;
    while at + BLOCK <= data.len() {
        let head = &data[at..at + BLOCK];
        /*
         * A zero block ends an archive, and an apk is several archives end to
         * end: a signature, a control stream, then the data.
         */
        if head.iter().all(|b| *b == 0) {
            at += BLOCK;
            continue;
        }
        let Some(size) = octal(&head[SIZE_AT..SIZE_AT + SIZE_LEN]) else {
            break;
        };
        let body_at = at + BLOCK;
        let end = body_at + size;
        if end > data.len() {
            break;
        }
        if head[TYPE_AT] == b'0' || head[TYPE_AT] == 0 {
            let name = name_of(head);
            out.push(Entry { name, body: data[body_at..end].to_vec() });
        }
        at = body_at + size.div_ceil(BLOCK) * BLOCK;
    }
    out
}
