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

use crate::syscall::{call_raw, N_CRYPTO_SECP256K1_SIGN};

/// Recoverable secp256k1 ECDSA sign of a 32-byte digest.
///
/// `sk` points to a 32-byte secret, `digest` to a 32-byte message hash,
/// `out` to a 65-byte buffer that receives r||s||v with v in {27,28}.
/// The secret is consumed and zeroized inside the kernel; it is never
/// returned. Returns 65 on success, negative errno otherwise.
#[no_mangle]
pub extern "C" fn crypto_secp256k1_sign(sk: *const u8, digest: *const u8, out: *mut u8) -> i64 {
    call_raw(N_CRYPTO_SECP256K1_SIGN, [sk as u64, digest as u64, out as u64, 0, 0, 0])
}
