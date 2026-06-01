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

use core::sync::atomic::AtomicU32;

pub(super) const RING_LEN: usize = 4096;
pub(super) const RING_MASK: u32 = (RING_LEN - 1) as u32;
pub(super) const MAGIC: u32 = u32::from_le_bytes(*b"NDBG");

#[repr(C, align(32))]
#[derive(Copy, Clone)]
pub(super) struct DbgEntry {
    pub(super) tag: u32,
    pub(super) ts: u32,
    pub(super) cpu: u32,
    pub(super) pid: u32,
    pub(super) payload: [u8; 16],
}

impl DbgEntry {
    pub(super) const ZERO: Self = Self { tag: 0, ts: 0, cpu: 0, pid: 0, payload: [0; 16] };
}

#[repr(C)]
pub(super) struct DbgRing {
    pub(super) magic: u32,
    pub(super) _pad: u32,
    pub(super) head: AtomicU32,
    pub(super) _pad2: u32,
    pub(super) entries: [DbgEntry; RING_LEN],
}
