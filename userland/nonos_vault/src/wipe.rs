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
//! Every secret this crate touches is a fixed-size array on the stack: the
//! machine key, the extracted pseudorandom key, the per-record subkey, the
//! plaintext on its way in or out. Each one is wiped before its scope ends,
//! because the next call in this capsule gets the same stack.

use core::sync::atomic::{compiler_fence, Ordering};

/// A volatile write per byte, so the compiler cannot elide stores to a buffer
/// it can see is never read again, which is exactly what this is.
///
/// The one unsafe block writes a byte of a slice this call holds mutably,
/// through a raw pointer only so the store is volatile.
pub(crate) fn wipe(buf: &mut [u8]) {
    for byte in buf.iter_mut() {
        unsafe { core::ptr::write_volatile(byte, 0) };
    }
    compiler_fence(Ordering::SeqCst);
}
