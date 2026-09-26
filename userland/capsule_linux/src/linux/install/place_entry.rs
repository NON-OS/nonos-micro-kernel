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

//! One file out of a package: where it lands, and whether the machine
//! says anything about it.

use nonos_libc::mk_debug;

use crate::linux::file::{key, store_write, visible};

use super::enrol::vouch;
use super::provenance::Provenance;
use super::tar::Entry;

/// Paths a package may not write: a package dropping one of these
/// would be installing its own proof.
const REFUSED: &[&[u8]] = &[b".nonos_id_cert.bin", b".manifest.bin", b".zk_trailer.bin"];

/// True when the file landed in the store.
pub(super) fn one(entry: &Entry, from: Provenance) -> bool {
    if entry.name.starts_with(b".") {
        return false;
    }
    if REFUSED.iter().any(|s| entry.name.ends_with(s)) {
        say(b"[LINUX] refused a package writing its own proof\n");
        return false;
    }
    let at = visible(b"/", &entry.name);
    if store_write(&key(&at), &entry.body).is_err() {
        return false;
    }
    if is_elf(&entry.body) {
        vouch_for(&at, &entry.body, from);
    }
    true
}

/// Minting says this machine agreed to run these bytes, so it is only said
/// about bytes something authenticated.
fn vouch_for(at: &[u8], body: &[u8], from: Provenance) {
    if from == Provenance::Unauthenticated {
        say(b"[LINUX] installed unvouched: package bytes are not authenticated\n");
        return;
    }
    if !vouch(at, body) {
        say(b"[LINUX] installed but unvouched: no enrolled root, or may not mint\n");
    }
}

/// A shared object counts: an interpreter loads it, and the gate
/// measures whatever it loads.
fn is_elf(body: &[u8]) -> bool {
    body.starts_with(b"\x7fELF")
}

fn say(line: &[u8]) {
    let _ = mk_debug(line.as_ptr(), line.len());
}
