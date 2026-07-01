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

pub const MAGIC_NTCP: u32 = 0x4E54_4350;

pub const OP_CONNECT: u16 = 3;
pub const OP_SEND: u16 = 5;
pub const OP_RECV: u16 = 6;
pub const OP_CLOSE: u16 = 7;
pub const OP_STATE: u16 = 9;

pub const E_OK: u16 = 0;
pub const E_BAD_OP: u16 = 3;
pub const E_BAD_LEN: u16 = 4;
pub const E_NO_SOCKET: u16 = 5;
pub const E_RX_EMPTY: u16 = 11;
pub const E_NOT_CONNECTED: u16 = 12;
pub const E_BAD_ADDR: u16 = 13;
