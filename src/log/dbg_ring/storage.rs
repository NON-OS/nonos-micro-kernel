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

use super::types::{DbgEntry, DbgRing, RING_LEN};

#[link_section = ".nonos.dbg_ring"]
#[no_mangle]
static mut RING: DbgRing = DbgRing {
    magic: 0,
    _pad: 0,
    head: AtomicU32::new(0),
    _pad2: 0,
    entries: [DbgEntry::ZERO; RING_LEN],
};

#[inline(always)]
pub(super) fn ring_mut() -> &'static mut DbgRing {
    unsafe { &mut *core::ptr::addr_of_mut!(RING) }
}

#[inline(always)]
pub(super) fn ring_ref() -> &'static DbgRing {
    unsafe { &*core::ptr::addr_of!(RING) }
}
