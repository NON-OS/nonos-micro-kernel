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

use super::flags::PteFlags;

#[derive(Debug, Clone, Copy)]
pub struct PageAttributes {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
    pub user: bool,
    pub global: bool,
}

impl PageAttributes {
    pub const fn kernel_code() -> Self {
        Self { read: true, write: false, execute: true, user: false, global: true }
    }

    pub const fn kernel_data() -> Self {
        Self { read: true, write: true, execute: false, user: false, global: true }
    }

    pub const fn kernel_rodata() -> Self {
        Self { read: true, write: false, execute: false, user: false, global: true }
    }

    pub const fn user_code() -> Self {
        Self { read: true, write: false, execute: true, user: true, global: false }
    }

    pub const fn user_data() -> Self {
        Self { read: true, write: true, execute: false, user: true, global: false }
    }

    pub const fn user_rodata() -> Self {
        Self { read: true, write: false, execute: false, user: true, global: false }
    }

    pub fn to_pte_flags(&self) -> PteFlags {
        let mut flags = PteFlags::new().valid().accessed();
        if self.read { flags = flags.readable(); }
        if self.write { flags = flags.writable().dirty(); }
        if self.execute { flags = flags.executable(); }
        if self.user { flags = flags.user(); }
        if self.global { flags = flags.global(); }
        flags
    }
}

impl Default for PageAttributes {
    fn default() -> Self {
        Self::kernel_data()
    }
}
