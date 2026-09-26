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

//! Fetching and joining the branch indexes.

use alloc::format;
use alloc::vec::Vec;

use super::http::get;
use super::index::Index;
use super::run::{ARCH, BRANCHES, HOST, PORT, RELEASE};
use super::tar::entries;

pub(super) fn load_index() -> Option<Index> {
    let mut all: Vec<u8> = Vec::new();
    for branch in BRANCHES {
        let path = format!("/alpine/{RELEASE}/{branch}/{ARCH}/APKINDEX.tar.gz");
        let raw = get(HOST, PORT, &path)?;
        let plain = nonos_inflate::gunzip(&raw)?;
        for entry in entries(&plain) {
            if entry.name.ends_with(b"APKINDEX") {
                all.extend_from_slice(&entry.body);
            }
        }
    }
    Some(Index::parse(&all))
}
