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

use super::super::layout;
use crate::memory::addr::{PhysAddr, VirtAddr};

#[inline]
pub fn phys_to_virt(phys: PhysAddr) -> Option<VirtAddr> {
    phys_to_virt_checked(phys)
}

#[inline]
pub fn virt_to_phys(virt: VirtAddr) -> Option<PhysAddr> {
    virt_to_phys_checked(virt)
}

#[inline]
pub fn phys_to_virt_checked(phys: PhysAddr) -> Option<VirtAddr> {
    let p = phys.as_u64();
    if p >= layout::DIRECTMAP_SIZE {
        return None;
    }
    Some(VirtAddr::new(layout::DIRECTMAP_BASE + p))
}

#[inline]
pub fn virt_to_phys_checked(virt: VirtAddr) -> Option<PhysAddr> {
    let v = virt.as_u64();
    if v < layout::DIRECTMAP_BASE {
        return None;
    }
    let off = v - layout::DIRECTMAP_BASE;
    if off >= layout::DIRECTMAP_SIZE {
        return None;
    }
    Some(PhysAddr::new(off))
}
