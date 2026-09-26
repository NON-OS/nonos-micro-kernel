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

//! Proving a program before running it.

use alloc::vec::Vec;

use nonos_libc::CapsuleVerifySummary;

use crate::linux::file::{key, store_read};

use super::attest_local;
use super::attest_paths::beside;
use super::attest_publisher;

/// A certificate, manifest and trailer are small; a refusal to read one
/// of this size is a malformed artifact rather than a large one.
const MAX_ARTIFACT: u32 = 1 << 22;

pub fn verify(path: &[u8], image: &[u8]) -> Result<CapsuleVerifySummary, &'static str> {
    let trailer = fetch(path, b".zk_trailer.bin")?;
    let Ok(cert) = fetch(path, b".nonos_id_cert.bin") else {
        return attest_local::verify(image, &trailer);
    };
    let manifest = fetch(path, b".manifest.bin")?;
    attest_publisher::verify(image, &cert, &manifest, &trailer)
}

/// The proof sits beside the program, inside the same root, so the suffix is
/// appended to the guest-visible path and the whole thing is confined once.
fn fetch(path: &[u8], suffix: &[u8]) -> Result<Vec<u8>, &'static str> {
    let at = beside(path, suffix);
    store_read(&key(&at), MAX_ARTIFACT).map_err(|_| "no proof beside the program")
}
