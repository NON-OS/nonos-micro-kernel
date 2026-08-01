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

extern crate alloc;

use alloc::vec;
use core::sync::atomic::{compiler_fence, Ordering};

use crate::capabilities::Capability;
use crate::crypto::ed25519::{sign, KeyPair};
use crate::syscall::dispatch::{errno, require_capability};
use crate::syscall::SyscallResult;
use crate::usercopy::{copy_from_user, copy_to_user};

/// Bound on the message so a caller cannot ask the kernel to allocate an
/// unbounded buffer. A voucher body is far under this.
const MAX_MSG: usize = 4096;

/// Volatile zeroization the optimizer cannot elide.
fn zeroize32(buf: &mut [u8; 32]) {
    for b in buf.iter_mut() {
        unsafe {
            core::ptr::write_volatile(b, 0);
        }
    }
    compiler_fence(Ordering::SeqCst);
}

/// ed25519 sign over an arbitrary message with a caller-held seed.
///
/// Args: seed ptr (32-byte private seed), msg ptr, msg len, out ptr
/// (64-byte signature). The seed is copied into a kernel buffer, the keypair
/// derived from it, and the seed zeroized before return; it never leaves the
/// kernel except as the signature. Deterministic per ed25519, so the same
/// (seed, message) always yields the same signature.
pub fn handle_crypto_ed25519_sign(
    seed_ptr: u64,
    msg_ptr: u64,
    msg_len: u64,
    out: u64,
) -> SyscallResult {
    if let Err(e) = require_capability(Capability::Crypto) {
        return e;
    }
    if seed_ptr == 0 || out == 0 || (msg_ptr == 0 && msg_len != 0) {
        return errno(22);
    }
    if msg_len as usize > MAX_MSG {
        return errno(22);
    }

    let mut seed = [0u8; 32];
    if copy_from_user(seed_ptr, &mut seed).is_err() {
        zeroize32(&mut seed);
        return errno(14);
    }

    let mut msg = vec![0u8; msg_len as usize];
    if msg_len != 0 && copy_from_user(msg_ptr, &mut msg).is_err() {
        zeroize32(&mut seed);
        return errno(14);
    }

    let kp = KeyPair::from_seed(seed);
    let sig = sign(&kp, &msg).to_bytes();
    // Seed no longer needed; wipe before anything can fault.
    zeroize32(&mut seed);

    if copy_to_user(out, &sig).is_err() {
        return errno(14);
    }
    SyscallResult { value: 64, capability_consumed: false, audit_required: true }
}
