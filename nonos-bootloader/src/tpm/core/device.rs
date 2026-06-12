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

#[repr(C)]
pub struct TmpDevice {
    pub base_addr: u64,
    pub locality: u8,
    pub active: bool,
    pub session_count: u8,
    pub max_sessions: u8,
    pub capabilities: u32,
}

impl TmpDevice {
    pub fn new(base_addr: u64) -> Self {
        Self {
            base_addr,
            locality: 0,
            active: false,
            session_count: 0,
            max_sessions: 3,
            capabilities: 0,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.active && self.locality != 0xFF
    }

    pub fn get_register(&self, offset: u16) -> u64 {
        self.base_addr + offset as u64
    }

    pub fn has_capability(&self, cap: u32) -> bool {
        (self.capabilities & cap) != 0
    }
}
