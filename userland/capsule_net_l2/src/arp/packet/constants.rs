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

pub const PACKET_LEN: usize = 28;
pub(super) const HW_ETHERNET: u16 = 1;
pub(super) const PROTO_IPV4: u16 = 0x0800;
pub(super) const HLEN_MAC: u8 = 6;
pub(super) const PLEN_IPV4: u8 = 4;
pub const OPER_REQUEST: u16 = 1;
pub const OPER_REPLY: u16 = 2;
