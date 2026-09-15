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

//! Scrubbing the root and the secret.

use core::sync::atomic::{compiler_fence, Ordering};

/// A volatile write per byte, so the compiler cannot drop stores to a buffer
/// it can see is never read again. The machine root and the account secret
/// both pass through this module's stack. The one unsafe block writes a byte
/// of an array this call holds mutably, through a raw pointer only so the
/// store is volatile.
pub(super) fn wipe32(buf: &mut [u8; 32]) {
    for byte in buf.iter_mut() {
        unsafe { core::ptr::write_volatile(byte, 0) };
    }
    compiler_fence(Ordering::SeqCst);
}
