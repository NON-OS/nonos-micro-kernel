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

use alloc::vec::Vec;

use super::summary::{decode, PkgSummary};
use super::wire::{call, EPROTO};

// Hand-synced with `capsule_installer/src/protocol/types.rs` and
// `capsule_desktop_shell/src/installer_client/constants.rs`.
const OP_PKG_QUERY: u16 = 6;
const OP_PKG_COMMIT: u16 = 7;
const OP_PKG_REMOVE: u16 = 8;

// The widest summary the installer can emit is 43 bytes of fixed fields plus
// a 64 byte name and a 64 byte namespace, so 256 bytes always holds a whole
// reply and never truncates one into a decode failure.
const QUERY_RX: usize = 256;

// Verify a package on disk and report what installing it would grant,
// without writing anything to the store.
pub(super) fn pkg_query(path: &[u8]) -> Result<PkgSummary, i32> {
    let mut rx = [0u8; QUERY_RX];
    let n = call(OP_PKG_QUERY, &len_prefixed(path), &mut rx)?;
    decode(&rx[8..n]).ok_or(EPROTO)
}

// Install the package at `path`, but only if it still hashes to the digest
// the user was shown and consented to.
pub(super) fn pkg_commit(path: &[u8], digest: &[u8; 32]) -> Result<(), i32> {
    let mut body = Vec::with_capacity(34 + path.len());
    body.extend_from_slice(digest);
    body.extend_from_slice(&len_prefixed(path));
    let mut rx = [0u8; 32];
    call(OP_PKG_COMMIT, &body, &mut rx).map(|_| ())
}

// Drop every artifact of an installed slug from the RAM tree and the store.
pub(super) fn pkg_remove(name: &[u8]) -> Result<(), i32> {
    let mut rx = [0u8; 32];
    call(OP_PKG_REMOVE, &len_prefixed(name), &mut rx).map(|_| ())
}

fn len_prefixed(v: &[u8]) -> Vec<u8> {
    let mut b = Vec::with_capacity(2 + v.len());
    b.extend_from_slice(&(v.len() as u16).to_le_bytes());
    b.extend_from_slice(v);
    b
}
