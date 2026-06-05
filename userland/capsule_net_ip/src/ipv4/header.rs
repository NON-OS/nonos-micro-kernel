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

use super::addr::Ipv4Addr;

pub const HDR_LEN_MIN: usize = 20;
pub const VERSION_4: u8 = 4;
pub const DEFAULT_TTL: u8 = 64;
pub const CHECKSUM_OFFSET: usize = 10;

// Parsed view of an inbound IPv4 packet. The capsule only keeps the
// fields the data path needs (src + dst + protocol). Other wire
// fields are validated during parse and then discarded; the egress
// path rebuilds them from scratch on each TX.
#[derive(Clone, Copy)]
pub struct Ipv4Header {
    pub protocol: u8,
    pub src: Ipv4Addr,
    pub dst: Ipv4Addr,
}
