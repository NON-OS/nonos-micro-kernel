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

use core::sync::atomic::{compiler_fence, Ordering};

use crate::capabilities::Capability;
use crate::syscall::dispatch::{errno, require_capability};
use crate::syscall::SyscallResult;
use crate::usercopy::{copy_from_user, copy_to_user};

/// Volatile zeroization that the optimizer cannot elide.
fn zeroize32(buf: &mut [u8; 32]) {
    for b in buf.iter_mut() {
        unsafe {
            core::ptr::write_volatile(b, 0);
        }
    }
    compiler_fence(Ordering::SeqCst);
}

/// secp256k1 recoverable ECDSA sign over a 32-byte digest.
///
/// Args: sk ptr (32-byte secret), digest ptr (32-byte message hash),
/// out ptr (65-byte: r||s||v, v in {27,28}).
///
/// The secret is copied into a kernel stack buffer, used, then zeroized
/// before return. The secret never leaves the kernel except as the
/// resulting signature. Deterministic (RFC-6979) + low-s normalized, so
/// the same (key, digest) always yields the same signature.
pub fn handle_crypto_secp256k1_sign(sk_ptr: u64, digest_ptr: u64, out: u64) -> SyscallResult {
    if let Err(e) = require_capability(Capability::Crypto) {
        return e;
    }
    if sk_ptr == 0 || digest_ptr == 0 || out == 0 {
        return errno(22);
    }

    let mut sk = [0u8; 32];
    let mut digest = [0u8; 32];
    if copy_from_user(sk_ptr, &mut sk).is_err() {
        zeroize32(&mut sk);
        return errno(14);
    }
    if copy_from_user(digest_ptr, &mut digest).is_err() {
        zeroize32(&mut sk);
        return errno(14);
    }

    let sig = match crate::crypto::secp256k1::sign(&sk, &digest) {
        Some(s) => s,
        None => {
            zeroize32(&mut sk);
            return errno(22);
        }
    };
    // Secret no longer needed; wipe immediately.
    zeroize32(&mut sk);

    let mut out_buf = [0u8; 65];
    out_buf[0..32].copy_from_slice(&sig.r);
    out_buf[32..64].copy_from_slice(&sig.s);
    // Ethereum / EIP-712 recovery byte: v = 27 + recovery_id (low-s normalized).
    out_buf[64] = 27u8.wrapping_add(sig.recovery_id & 1);

    if copy_to_user(out, &out_buf).is_err() {
        return errno(14);
    }
    SyscallResult { value: 65, capability_consumed: false, audit_required: true }
}
