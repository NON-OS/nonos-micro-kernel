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

//! A device register window, in host memory.
//!
//! Driver capsules reach their registers through a small type that holds the
//! window's base address and does volatile accesses at offsets from it. That
//! type takes the base as a plain integer, which is the whole reason this
//! works: hand it the address of a buffer here and every register path in the
//! driver runs on the host, unmodified, with no device present.
//!
//! The backing store is `AtomicU8` rather than plain bytes, and not for the
//! atomicity. It is because a device model runs concurrently with the driver
//! (see [`super::live`]), so the test side of every access has to be defined
//! when another thread is touching the same window. `AtomicU8` has the layout
//! of `u8`, so the driver's raw volatile accesses through `base()` see exactly
//! the bytes they would have seen.

use core::sync::atomic::AtomicU8;

pub struct FakeBar {
    cells: Box<[AtomicU8]>,
}

impl FakeBar {
    /// A window of `len` bytes, reading as zero.
    pub fn new(len: usize) -> Self {
        let mut v = Vec::with_capacity(len);
        v.resize_with(len, || AtomicU8::new(0));
        Self { cells: v.into_boxed_slice() }
    }

    /// The address to hand the driver's register type.
    ///
    /// The driver writes through this while the test still holds `&self`. That
    /// is the arrangement being modelled: hardware and software share a
    /// register file, and neither waits for the other.
    pub fn base(&self) -> u64 {
        self.cells.as_ptr() as u64
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    /// Bounds-checked by indexing, so an offset past the window panics in the
    /// test rather than reading whatever follows the allocation.
    pub(crate) fn cell(&self, offset: usize) -> &AtomicU8 {
        &self.cells[offset]
    }
}
