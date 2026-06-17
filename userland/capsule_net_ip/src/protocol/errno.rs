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

//! Wire errnos returned in the response header `errno` field.
//! Numbers are part of the wire contract — never renumber a live
//! value, only retire and add. Clients mirror these by number, so
//! any change here must be reflected wherever the wire is parsed.

pub const E_OK: u16 = 0;
pub const E_BAD_MAGIC: u16 = 1;
pub const E_BAD_VERSION: u16 = 2;
pub const E_BAD_OP: u16 = 3;
pub const E_BAD_LEN: u16 = 4;
pub const E_NO_CONFIG: u16 = 5;
pub const E_NO_ROUTE: u16 = 6;
pub const E_NO_NEIGHBOUR: u16 = 7;
pub const E_L2_FAULT: u16 = 8;
pub const E_BAD_PACKET: u16 = 9;
pub const E_RX_EMPTY: u16 = 10;
pub const E_TABLE_FULL: u16 = 11;
pub const E_PERM: u16 = 12;
