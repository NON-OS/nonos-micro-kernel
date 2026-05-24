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

use core::cell::Cell;

use super::resource::Resource;
use crate::protocol::MAX_RESOURCES;

pub struct ResourceTable {
    next_id: Cell<u32>,
    entries: [Cell<Resource>; MAX_RESOURCES],
}

impl ResourceTable {
    pub const fn new() -> Self {
        const EMPTY: Cell<Resource> = Cell::new(Resource {
            resource_id: 0,
            owner_pid: 0,
            width: 0,
            height: 0,
            format: 0,
            backing_addr: 0,
            backing_len: 0,
            in_use: false,
        });
        Self { next_id: Cell::new(1), entries: [EMPTY; MAX_RESOURCES] }
    }
    pub fn alloc_id(&self) -> u32 {
        let id = self.next_id.get();
        self.next_id.set(id.wrapping_add(1).max(1));
        id
    }
    pub fn insert(&self, r: Resource) -> Result<(), ()> {
        for entry in &self.entries {
            if !entry.get().in_use {
                entry.set(r);
                return Ok(());
            }
        }
        Err(())
    }
    pub fn lookup(&self, resource_id: u32) -> Option<Resource> {
        for entry in &self.entries {
            let r = entry.get();
            if r.in_use && r.resource_id == resource_id {
                return Some(r);
            }
        }
        None
    }
    pub fn update<F: FnOnce(&mut Resource)>(&self, resource_id: u32, f: F) -> bool {
        for entry in &self.entries {
            let mut r = entry.get();
            if r.in_use && r.resource_id == resource_id {
                f(&mut r);
                entry.set(r);
                return true;
            }
        }
        false
    }
}
