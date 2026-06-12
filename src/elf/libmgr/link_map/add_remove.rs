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

use alloc::{boxed::Box, vec::Vec};

use crate::memory::addr::VirtAddr;

use super::{entry::LinkMapEntry, state::LinkMap};

impl LinkMap {
    pub fn add(
        &mut self,
        base_addr: VirtAddr,
        name: &str,
        dynamic_addr: VirtAddr,
    ) -> *mut LinkMapEntry {
        let mut name_bytes: Vec<u8> = name.bytes().collect();
        name_bytes.push(0);
        self.names.push(name_bytes);
        let name_ptr = self.names.last().map_or(core::ptr::null(), |name| name.as_ptr());
        let mut entry =
            Box::new(LinkMapEntry::new(base_addr.as_u64(), name_ptr, dynamic_addr.as_u64()));
        let entry_ptr = entry.as_mut() as *mut LinkMapEntry;
        if self.head.is_null() {
            self.head = entry_ptr;
            self.tail = entry_ptr;
        } else {
            unsafe {
                (*self.tail).l_next = entry_ptr;
                entry.l_prev = self.tail;
            }
            self.tail = entry_ptr;
        }
        self.entries.push(entry);
        entry_ptr
    }

    pub fn remove(&mut self, base_addr: VirtAddr) -> bool {
        let Some(index) = self.entries.iter().position(|entry| entry.l_addr == base_addr.as_u64())
        else {
            return false;
        };
        let entry = &self.entries[index];
        unsafe {
            if !entry.l_prev.is_null() {
                (*entry.l_prev).l_next = entry.l_next;
            } else {
                self.head = entry.l_next;
            }
            if !entry.l_next.is_null() {
                (*entry.l_next).l_prev = entry.l_prev;
            } else {
                self.tail = entry.l_prev;
            }
        }
        self.entries.remove(index);
        self.names.remove(index);
        true
    }
}
