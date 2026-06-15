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

//! `net.l2` v1 envelope as the DHCP capsule sees it. The DHCP
//! client uses raw `OP_SEND_FRAME` and `OP_POLL_FRAME` because the
//! protocol predates the IP layer: it has to ship and receive
//! broadcasts before an IPv4 address exists.

pub const L2_MAGIC: u32 = 0x4E4C_3200; // "NL2\0"
pub const L2_VERSION: u16 = 1;
pub const L2_HDR_LEN: usize = 20;

pub const OP_GET_MAC: u16 = 2;
pub const OP_SEND_FRAME: u16 = 4;
pub const OP_POLL_FRAME: u16 = 5;
pub const OP_SET_IP: u16 = 7;
