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

pub const MAGIC_NDNS: u32 = 0x4E44_4E53;

pub const OP_RESOLVE_A: u16 = 2;

pub const E_OK: u16 = 0;
pub const E_BAD_OP: u16 = 3;
pub const E_NAME_INVALID: u16 = 9;
pub const E_SERVFAIL: u16 = 10;
pub const E_NO_LEASE: u16 = 11;
