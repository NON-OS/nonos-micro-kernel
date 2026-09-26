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

//! Fetching one package, from whichever branch holds it.

use alloc::format;
use alloc::vec::Vec;

use super::http::get;
use super::run::{ARCH, BRANCHES, HOST, PORT, RELEASE};

/// Either branch may hold it, and the index does not say which.
pub(super) fn download(name: &str, version: &str) -> Vec<u8> {
    for branch in BRANCHES {
        let path = format!("/alpine/{RELEASE}/{branch}/{ARCH}/{name}-{version}.apk");
        if let Some(bytes) = get(HOST, PORT, &path) {
            return bytes;
        }
    }
    Vec::new()
}
