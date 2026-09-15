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

//! The record on disk, and why every byte of it is there.

use nonos_seal::{NONCE_LEN, TAG_LEN};

/// Recognisable in a hexdump of a partition, and enough to refuse a file that
/// is not one of these before any key is derived.
pub const MAGIC: [u8; 8] = *b"NONOSVLT";

/// Two. Version one was the wallet-only format that sealed under the machine
/// key directly; this one derives a key per record, so a v1 blob must fail
/// cleanly rather than be opened under the wrong rules.
pub const VERSION: u16 = 2;

/// Magic 8, version 2, record-name length 1, reserved 1.
pub const HEADER_LEN: usize = 12;

/// What sealing adds to the plaintext: header, nonce, tag.
pub const OVERHEAD: usize = HEADER_LEN + NONCE_LEN + TAG_LEN;

pub const NONCE_AT: usize = HEADER_LEN;
pub const CIPHERTEXT_AT: usize = HEADER_LEN + NONCE_LEN;

/// The header, which is also the associated data.
///
/// On the way out this is what gets written and authenticated. On the way in
/// the authenticated bytes are the ones from the record itself, never a
/// rebuilt copy: rebuilding authenticates what the reader expected rather
/// than what it was given, so any header byte the parser does not explicitly
/// compare would be free for an attacker to change. That is exactly what
/// happened to the reserved byte, which nothing compared.
pub fn header(record_len: u8) -> [u8; HEADER_LEN] {
    let mut h = [0u8; HEADER_LEN];
    h[..8].copy_from_slice(&MAGIC);
    h[8..10].copy_from_slice(&VERSION.to_le_bytes());
    h[10] = record_len;
    h
}
