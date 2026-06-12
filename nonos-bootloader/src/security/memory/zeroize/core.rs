// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

#[inline(never)]
pub fn zeroize_slice(data: &mut [u8]) {
    for b in data.iter_mut() {
        unsafe {
            ptr::write_volatile(b, 0);
        }
    }
    compiler_fence(Ordering::SeqCst);
}

#[inline(never)]
pub fn zeroize_32(data: &mut [u8; 32]) {
    for b in data.iter_mut() {
        unsafe {
            ptr::write_volatile(b, 0);
        }
    }
    compiler_fence(Ordering::SeqCst);
}

#[inline(never)]
pub fn zeroize_64(data: &mut [u8; 64]) {
    for b in data.iter_mut() {
        unsafe {
            ptr::write_volatile(b, 0);
        }
    }
    compiler_fence(Ordering::SeqCst);
}

#[inline(never)]
pub fn zeroize_128(data: &mut [u8; 128]) {
    for b in data.iter_mut() {
        unsafe {
            ptr::write_volatile(b, 0);
        }
    }
    compiler_fence(Ordering::SeqCst);
}
