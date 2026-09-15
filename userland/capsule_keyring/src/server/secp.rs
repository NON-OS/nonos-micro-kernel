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

//! secp256k1, from the library rather than from the kernel.
//!
//! Every signature this capsule produced used to be a syscall, which put the
//! curve's point, field and scalar arithmetic in ring 0 for the benefit of one
//! application. It is a library now, in this process, holding no privilege it
//! does not need. The keyring was already the only thing that asked.
//!
//! The two functions keep the syscall's return contract, 65 on success and a
//! negative number otherwise, so the eleven call sites changed only in which
//! name they call. Signatures are byte-identical to what the kernel produced,
//! which `crypto_proofs` checks by running both over the same inputs.

use nonos_secp256k1::{public_key_from_secret, sign as ecdsa_sign};

/// Ethereum's recovery id offset. The kernel syscall returned `v` already
/// biased by 27 and the callers subtract it again, so the bias stays here
/// rather than becoming a change every one of them has to notice.
const ETH_V_OFFSET: u8 = 27;

const OK: i64 = 65;
const FAILED: i64 = -1;

/// `r || s || v`, with `v` biased by 27, exactly as the syscall returned it.
pub fn sign(secret: &[u8; 32], digest: &[u8; 32], out: &mut [u8; 65]) -> i64 {
    match ecdsa_sign(secret, digest) {
        Some(sig) => {
            out[0..32].copy_from_slice(&sig.r);
            out[32..64].copy_from_slice(&sig.s);
            out[64] = sig.recovery_id + ETH_V_OFFSET;
            OK
        }
        None => FAILED,
    }
}

/// The uncompressed public key, 0x04 then x then y.
pub fn pubkey(secret: &[u8; 32], out: &mut [u8; 65]) -> i64 {
    match public_key_from_secret(secret) {
        Some(pk) => {
            out.copy_from_slice(&pk);
            OK
        }
        None => FAILED,
    }
}
