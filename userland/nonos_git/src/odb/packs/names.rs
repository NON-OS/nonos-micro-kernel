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
//! Which packs a repository holds.

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::storage::Storage;

/// The base name of every pack, without the extension.
///
/// A pack is only usable with its index, so a `.pack` whose `.idx` is missing
/// is skipped rather than reported: it is what a half-finished fetch leaves
/// behind, and the objects are not reachable through it either way.
pub(super) fn pack_names<S: Storage>(storage: &S, git_dir: &str) -> Vec<String> {
    let dir = format!("{git_dir}/objects/pack");
    let mut out = Vec::new();
    let Ok(entries) = storage.read_dir(&dir) else {
        return out;
    };
    for entry in entries {
        let Some(base) = entry.strip_suffix(".idx") else {
            continue;
        };
        if storage.exists(&format!("{dir}/{base}.pack")) {
            out.push(format!("{dir}/{base}"));
        }
    }
    out
}
