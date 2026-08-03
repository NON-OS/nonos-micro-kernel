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

/// Ed25519 public key from a 32-byte seed.
///
/// A public key cannot be recovered from a signature, so without this a
/// capsule holding a seed has no way to learn its own identity and has to be
/// handed the pair by whoever configured it. The seed is copied in, used, and
/// zeroized before return; it never leaves the caller except as the key it
/// determines.
pub fn handle_crypto_ed25519_pubkey(seed_ptr: u64, out: u64) -> SyscallResult {
    if let Err(e) = require_capability(Capability::Crypto) {
        return e;
    }
    if seed_ptr == 0 || out == 0 {
        return errno(22);
    }
    let mut seed = [0u8; 32];
    if copy_from_user(seed_ptr, &mut seed).is_err() {
        zeroize(&mut seed);
        return errno(14);
    }
    let public = crate::crypto::asymmetric::ed25519::pubkey_from_secret(&seed);
    zeroize(&mut seed);
    if copy_to_user(out, &public).is_err() {
        return errno(14);
    }
    SyscallResult { value: 32, capability_consumed: false, audit_required: true }
}

/// Volatile wipe the optimizer cannot elide.
fn zeroize(buf: &mut [u8; 32]) {
    for b in buf.iter_mut() {
        unsafe { core::ptr::write_volatile(b, 0) };
    }
    compiler_fence(Ordering::SeqCst);
}
