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

pub const E_AGAIN: i32 = -11;
pub const HDR_LEN: usize = 20;
pub const MAGIC: u32 = 0x4E58_4843;
pub const OP_ADDRESS_DEVICE: u16 = 0x0006;
pub const OP_ALLOC_TRANSFER_RING: u16 = 0x0009;
pub const OP_CONTROL_TRANSFER: u16 = 0x000B;
pub const OP_ENABLE_SLOT: u16 = 0x0004;
pub const OP_GET_CONFIG_DESCRIPTOR: u16 = 0x0008;
pub const OP_INTERRUPT_IN: u16 = 0x000E;
pub const OP_PORT_STATUS: u16 = 0x0003;
pub const STATUS_LEN: usize = 4;
pub const VERSION: u16 = 1;
