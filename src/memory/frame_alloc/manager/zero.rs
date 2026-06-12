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

use super::super::constants::FRAME_SIZE;
use crate::memory::addr::PhysAddr;
use crate::memory::layout::{DIRECTMAP_BASE, DIRECTMAP_SIZE};

pub fn zero_frame(addr: PhysAddr) {
    let pa = addr.as_u64();
    if pa >= DIRECTMAP_SIZE {
        return;
    }
    let va = DIRECTMAP_BASE.wrapping_add(pa);
    if va < DIRECTMAP_BASE
        || va.wrapping_add(FRAME_SIZE) > DIRECTMAP_BASE.wrapping_add(DIRECTMAP_SIZE)
    {
        return;
    }
    unsafe {
        core::ptr::write_bytes(va as *mut u8, 0, FRAME_SIZE as usize);
    }
}
