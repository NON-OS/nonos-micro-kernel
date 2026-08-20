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

//! Turn a LIST reply into the direct children of `prefix`. The framing walk and
//! the per-name decision live in their own files; this just wires them together
//! and adapts the result to `Entry`.

use alloc::vec::Vec;

use super::classify::classify;
use super::constants::HDR_LEN;
use super::entry::Entry;
use super::under::{count_under, relative};
use super::walk::walk;

pub(super) fn children(prefix: &str, rx: &[u8], total: usize) -> Vec<Entry> {
    let rels = walk(rx, HDR_LEN + 4, total, |raw| relative(prefix, raw));
    walk(rx, HDR_LEN + 4, total, |raw| classify(prefix, raw))
        .into_iter()
        .map(|(name, is_dir)| {
            let children = if is_dir { count_under(&rels, &name) } else { 0 };
            Entry { name, is_dir, children }
        })
        .collect()
}
