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

// Overwrite a buffer with zeros before its memory is freed or truncated, so a
// deleted file's contents cannot linger in reclaimed heap. Volatile writes plus
// a compiler fence stop the optimiser from eliding the erase as a dead store.
pub(super) fn zeroize(buf: &mut [u8]) {
    for byte in buf.iter_mut() {
        // SAFETY: ek@nonos.systems — `byte` is a valid, uniquely borrowed,
        // aligned u8 owned by this buffer; a volatile write of 0 is in bounds.
        unsafe {
            core::ptr::write_volatile(byte, 0);
        }
    }
    compiler_fence(Ordering::SeqCst);
}
