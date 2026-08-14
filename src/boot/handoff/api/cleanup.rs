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

use core::ptr;
use core::sync::atomic::{compiler_fence, Ordering};

use super::super::types::BootHandoffV1;
use super::query::BOOT_HANDOFF;

/// Clear the bootloader's entropy contribution once the kernel CSPRNG has
/// taken it.
///
/// Only the seed. The measurements and the ZK attestation in the handoff are
/// served for the life of the system by the `MkAttestStatus` syscall, so
/// wiping them here would break attestation rather than protect anything: they
/// are public values a capsule is entitled to read. The seed is the one field
/// that is secret, is consumed exactly once, and has no reason to outlive that.
pub fn wipe_boot_seed() {
    if let Some(&handoff) = BOOT_HANDOFF.get() {
        let ptr = handoff as *const BootHandoffV1 as *mut BootHandoffV1;
        if ptr.is_null() {
            return;
        }
        // SAFETY: eK@nonos.systems - `BOOT_HANDOFF` holds the bootloader's
        // handoff page, mapped for the life of the kernel, and this is the
        // only writer of the seed field after the CSPRNG has consumed it.
        unsafe {
            wipe_seed(&mut (*ptr).rng.seed32);
        }
        compiler_fence(Ordering::SeqCst);
    }
}

/// Volatile so the writes survive an optimiser that can see the seed is never
/// read again.
fn wipe_seed(seed: &mut [u8; 32]) {
    for b in seed.iter_mut() {
        // SAFETY: eK@nonos.systems - `b` is a live byte of the handoff seed
        // array borrowed mutably above.
        unsafe {
            ptr::write_volatile(b, 0);
        }
    }
}
