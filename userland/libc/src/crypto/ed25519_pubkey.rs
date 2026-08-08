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

//! Userland wrapper for `CryptoEd25519Pubkey`. The kernel cap-gates by
//! `Capability::Crypto`, derives the key, and zeroizes the seed before
//! returning.

use crate::syscall::{call_raw, N_CRYPTO_ED25519_PUBKEY};

/// Derive the 32-byte Ed25519 public key for `seed`, writing it to `out`.
/// Returns 32 on success, negative errno otherwise.
///
/// A public key cannot be recovered from a signature, so a capsule that holds
/// only a seed needs this to learn its own identity.
#[no_mangle]
pub extern "C" fn crypto_ed25519_pubkey(seed: *const u8, out: *mut u8) -> i64 {
    call_raw(N_CRYPTO_ED25519_PUBKEY, [seed as u64, out as u64, 0, 0, 0, 0])
}
