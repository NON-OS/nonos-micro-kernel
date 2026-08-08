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

//! Userland wrapper for `CryptoEd25519Sign`. The kernel cap-gates by
//! `Capability::Crypto`, derives the keypair from the seed, signs, and
//! zeroizes the seed before returning; the seed never leaves the caller's
//! address space except as the 64-byte signature.

use crate::syscall::{call_raw, N_CRYPTO_ED25519_SIGN};

/// Sign `message` with the 32-byte `seed`, writing a 64-byte signature to
/// `out`. Returns 64 on success, negative errno otherwise. Deterministic: the
/// same seed and message always produce the same signature.
///
/// Argument order differs from `crypto_ed25519_verify`, which takes the public
/// key and the signature first.
#[no_mangle]
pub extern "C" fn crypto_ed25519_sign(
    seed: *const u8,
    message: *const u8,
    message_len: usize,
    out: *mut u8,
) -> i64 {
    call_raw(
        N_CRYPTO_ED25519_SIGN,
        [seed as u64, message as u64, message_len as u64, out as u64, 0, 0],
    )
}
