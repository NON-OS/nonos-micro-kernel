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

pub const MAGIC_NNET: u32 = 0x4E4E_4554;
pub const MAGIC_NDHC: u32 = 0x4E44_4843;
pub const VERSION: u16 = 1;
pub const HDR_LEN: usize = 20;

pub const OP_MAC_ADDRESS: u16 = 3;
pub const OP_TX_PACKET: u16 = 4;
pub const OP_RX_PACKET: u16 = 5;

pub const OP_LEASE_STATUS: u16 = 3;
