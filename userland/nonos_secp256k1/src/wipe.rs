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

//! Scrubbing a buffer that held key material.
//!
//! `Scalar` and `FieldElement` zeroize themselves on drop, so the nonce and
//! the private scalar are covered. The byte buffers around them were not: RFC
//! 6979 builds `V || 0x00 || key || hash` in a 97-byte array, so the private
//! key sits in plain bytes on the stack, and the function used to return
//! without touching it. That is a copy of the key left where the next call in
//! this capsule reuses the stack.

use core::sync::atomic::{compiler_fence, Ordering};

/// A volatile write per byte, so the compiler cannot elide stores to a buffer
/// it can see is never read again, which is exactly what this is.
pub(crate) fn wipe(buf: &mut [u8]) {
    for byte in buf.iter_mut() {
        /*
         * SAFETY: ek@nonos.systems - a byte inside a slice this call owns
         * mutably, written through a raw pointer only to defeat the
         * optimiser's dead-store elimination.
         */
        unsafe { core::ptr::write_volatile(byte, 0) };
    }
    compiler_fence(Ordering::SeqCst);
}
