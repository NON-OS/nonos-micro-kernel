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
//! Putting a package's files into the store, under the Linux root.

use nonos_inflate::gunzip;
use nonos_libc::mk_debug;

use super::place_entry::one;
use super::provenance::Provenance;
use super::tar::entries;

/// Unpack `apk` into the store and report how many files landed.
pub fn unpack(apk: &[u8], from: Provenance) -> usize {
    let Some(raw) = gunzip(apk) else {
        say(b"[LINUX] package is not readable\n");
        return 0;
    };
    entries(&raw).iter().filter(|entry| one(entry, from)).count()
}

fn say(line: &[u8]) {
    let _ = mk_debug(line.as_ptr(), line.len());
}
