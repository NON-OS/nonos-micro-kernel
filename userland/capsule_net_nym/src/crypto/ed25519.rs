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

//! ed25519, from the library rather than from the kernel.
//!
//! This capsule is what used those syscalls: it signs the gateway handshake,
//! derives its own identity key, and verifies the directory's signatures over
//! the topology. All of it went through ring 0, which put the curve's point,
//! field and scalar arithmetic in the kernel to serve one capsule.
//!
//! The kernel keeps ed25519 verification for the boot chain, because deciding
//! whether a capsule may load is the one signature check that cannot live
//! outside the thing doing the loading.
//!
//! These keep the syscalls' return contracts, so the call sites changed only
//! in the name they call, and signatures are byte-identical to what the kernel
//! produced, which `crypto_proofs` checked over sixty-four seeds before the
//! syscalls were retired. Verification lives in [`super::ed25519_verify`],
//! whose contract inverts and is worth reading before touching.

use nonos_ed25519::{pubkey_from_secret, sign as ed_sign, KeyPair};

pub const SIGNATURE_LEN: usize = 64;
pub const PUBLIC_LEN: usize = 32;

/// Signature length on success, negative on failure, as the syscall returned.
pub fn sign(seed: &[u8; 32], message: &[u8], out: &mut [u8; SIGNATURE_LEN]) -> i64 {
    let keypair = KeyPair::from_seed(*seed);
    let sig = ed_sign(&keypair, message);
    out.copy_from_slice(&sig.to_bytes());
    SIGNATURE_LEN as i64
}

/// Public key length on success, as the syscall returned.
pub fn pubkey(seed: &[u8; 32], out: &mut [u8; PUBLIC_LEN]) -> i64 {
    out.copy_from_slice(&pubkey_from_secret(seed));
    PUBLIC_LEN as i64
}
