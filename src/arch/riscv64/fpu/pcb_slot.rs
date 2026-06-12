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

use core::cell::UnsafeCell;

use super::slot::FpSlot;

#[repr(transparent)]
pub struct PcbArchFpu {
    inner: UnsafeCell<FpSlot>,
}

unsafe impl Sync for PcbArchFpu {}

impl PcbArchFpu {
    pub const fn zeroed() -> Self {
        Self { inner: UnsafeCell::new(FpSlot::zeroed()) }
    }

    pub fn slot_ptr(&self) -> *mut FpSlot {
        self.inner.get()
    }
}
