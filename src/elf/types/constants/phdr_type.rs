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

pub const PT_NULL: u32 = 0;
pub const PT_LOAD: u32 = 1;
pub const PT_DYNAMIC: u32 = 2;
pub const PT_INTERP: u32 = 3;
pub const PT_NOTE: u32 = 4;
pub const PT_SHLIB: u32 = 5;
pub const PT_PHDR: u32 = 6;
pub const PT_TLS: u32 = 7;
pub const PT_GNU_EH_FRAME: u32 = 0x6474_E550;
pub const PT_GNU_STACK: u32 = 0x6474_E551;
pub const PT_GNU_RELRO: u32 = 0x6474_E552;
pub const NULL: u32 = 0;
pub const LOAD: u32 = 1;
pub const DYNAMIC: u32 = 2;
pub const INTERP: u32 = 3;
pub const NOTE: u32 = 4;
pub const PHDR: u32 = 6;
pub const TLS: u32 = 7;
