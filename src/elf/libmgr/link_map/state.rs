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

extern crate alloc;

use alloc::{boxed::Box, vec::Vec};

use super::entry::LinkMapEntry;

pub struct LinkMap {
    pub(super) entries: Vec<Box<LinkMapEntry>>,
    pub(super) names: Vec<Vec<u8>>,
    pub(super) head: *mut LinkMapEntry,
    pub(super) tail: *mut LinkMapEntry,
}

impl LinkMap {
    pub fn new() -> Self { Self { entries: Vec::new(), names: Vec::new(), head: core::ptr::null_mut(), tail: core::ptr::null_mut() } }
    pub fn clear(&mut self) { self.entries.clear(); self.names.clear(); self.head = core::ptr::null_mut(); self.tail = core::ptr::null_mut(); }
}

impl Default for LinkMap {
    fn default() -> Self { Self::new() }
}

unsafe impl Send for LinkMap {}
