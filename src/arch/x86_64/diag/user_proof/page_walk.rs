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

//! Walk a target CR3 through the directmap and return either a
//! decoded leaf or a permission-checked `bool`. Never dereferences
//! a user VA: the kernel reaches every page-table page through
//! `DIRECTMAP_BASE + phys`, so a half-built user address space
//! cannot fault the audit itself.

use crate::memory::layout::DIRECTMAP_BASE;

const PRESENT: u64 = 1 << 0;
const WRITABLE: u64 = 1 << 1;
const USER: u64 = 1 << 2;
const HUGE: u64 = 1 << 7;
const NX: u64 = 1 << 63;
const PHYS_MASK: u64 = 0x000F_FFFF_FFFF_F000;
const PAGE_2M_MASK: u64 = 0x000F_FFFF_FFE0_0000;
const PAGE_1G_MASK: u64 = 0x000F_FFFF_C000_0000;

#[derive(Clone, Copy)]
pub(super) struct LeafReq {
    pub user: bool,
    pub writable: bool,
    pub executable: bool,
}

pub(super) struct Leaf {
    pub entry: u64,
    pub phys_base: u64,
    pub offset: u64,
}

pub(super) fn pml4_present(cr3: u64, idx: usize) -> bool {
    let pml4 = (DIRECTMAP_BASE + (cr3 & PHYS_MASK)) as *const u64;
    let entry = unsafe { core::ptr::read_volatile(pml4.add(idx)) };
    entry & PRESENT != 0
}

pub(super) fn leaf_satisfies(cr3: u64, va: u64, req: LeafReq) -> bool {
    let Some(leaf) = leaf_for(cr3, va) else {
        return false;
    };
    matches_perms(leaf.entry, req)
}

pub(super) fn leaf_for(cr3: u64, va: u64) -> Option<Leaf> {
    let i4 = ((va >> 39) & 0x1FF) as usize;
    let i3 = ((va >> 30) & 0x1FF) as usize;
    let i2 = ((va >> 21) & 0x1FF) as usize;
    let i1 = ((va >> 12) & 0x1FF) as usize;

    let pml4 = (DIRECTMAP_BASE + (cr3 & PHYS_MASK)) as *const u64;
    let e4 = unsafe { core::ptr::read_volatile(pml4.add(i4)) };
    if e4 & PRESENT == 0 {
        return None;
    }
    let e3_tbl = (DIRECTMAP_BASE + (e4 & PHYS_MASK)) as *const u64;
    let e3 = unsafe { core::ptr::read_volatile(e3_tbl.add(i3)) };
    if e3 & PRESENT == 0 {
        return None;
    }
    if e3 & HUGE != 0 {
        return Some(Leaf { entry: e3, phys_base: e3 & PAGE_1G_MASK, offset: va & 0x3FFF_FFFF });
    }
    let e2_tbl = (DIRECTMAP_BASE + (e3 & PHYS_MASK)) as *const u64;
    let e2 = unsafe { core::ptr::read_volatile(e2_tbl.add(i2)) };
    if e2 & PRESENT == 0 {
        return None;
    }
    if e2 & HUGE != 0 {
        return Some(Leaf { entry: e2, phys_base: e2 & PAGE_2M_MASK, offset: va & 0x1F_FFFF });
    }
    let e1_tbl = (DIRECTMAP_BASE + (e2 & PHYS_MASK)) as *const u64;
    let e1 = unsafe { core::ptr::read_volatile(e1_tbl.add(i1)) };
    if e1 & PRESENT == 0 {
        return None;
    }
    Some(Leaf { entry: e1, phys_base: e1 & PHYS_MASK, offset: va & 0xFFF })
}

fn matches_perms(entry: u64, req: LeafReq) -> bool {
    if req.user && entry & USER == 0 {
        return false;
    }
    if req.writable && entry & WRITABLE == 0 {
        return false;
    }
    if req.executable && entry & NX != 0 {
        return false;
    }
    if !req.executable && entry & NX == 0 {
        return false;
    }
    true
}
