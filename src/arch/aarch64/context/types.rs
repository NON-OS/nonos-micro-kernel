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
#[derive(Clone, Copy, Debug, Default)]
pub struct UserEntry {
    pub entry: u64,

    pub user_sp: u64,

    pub spsr: u64,

    pub kernel_sp: u64,

    pub args: [u64; 8],
}

impl UserEntry {
    pub const fn zeroed() -> Self {
        Self { entry: 0, user_sp: 0, spsr: 0, kernel_sp: 0, args: [0; 8] }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SavedUser {
    pub gprs: [u64; 31],

    pub sp_el0: u64,

    pub elr_el1: u64,

    pub spsr_el1: u64,

    pub kernel_sp: u64,
}

impl SavedUser {
    pub const fn zeroed() -> Self {
        Self { gprs: [0; 31], sp_el0: 0, elr_el1: 0, spsr_el1: 0, kernel_sp: 0 }
    }
}
