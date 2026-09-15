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

//! The pointer arguments of the libc ABI, turned into references once.
//!
//! The real `nonos_libc` hands these pointers to the kernel as integers and
//! never dereferences them; the kernel checks them against the caller's
//! address space. This shim is the kernel's side too, so it is the place
//! they become memory, and the contract is the one every caller of the real
//! ABI already keeps: a pointer names `len` live, suitably aligned bytes of
//! the caller's own for the duration of the call.

pub(crate) fn bytes<'a>(ptr: *const u8, len: usize) -> &'a [u8] {
    /*
     * SAFETY: the ABI contract above; the driver passes its own buffer and
     * the length it filled.
     */
    unsafe { std::slice::from_raw_parts(ptr, len) }
}

pub(crate) fn bytes_mut<'a>(ptr: *mut u8, len: usize) -> &'a mut [u8] {
    /*
     * SAFETY: the ABI contract above; the driver passes a buffer it owns and
     * reads back from after the call.
     */
    unsafe { std::slice::from_raw_parts_mut(ptr, len) }
}

pub(crate) fn load<T: Copy>(ptr: *const T) -> T {
    /*
     * SAFETY: the ABI contract above; the driver passes a reference to a live
     * value of `T`.
     */
    unsafe { *ptr }
}

pub(crate) fn store<T>(ptr: *mut T, value: T) {
    /*
     * SAFETY: the ABI contract above; the driver passes a reference to a live
     * output of `T` it reads after the call.
     */
    unsafe { *ptr = value }
}
