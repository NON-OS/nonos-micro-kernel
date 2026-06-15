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

pub const DNS_MAGIC: u32 = 0x4E44_4E53;
pub const ETH_RPC_HOST: &[u8] = b"ethereum-rpc.publicnode.com";
pub const NYM_MAGIC: u32 = 0x4E59_4D31;
pub const OP_HEALTHCHECK: u16 = 1;
pub const OP_RESOLVE_A: u16 = 2;
pub const OP_SOCKET: u16 = 2;
pub const OP_CONNECT: u16 = 6;
pub const OP_SEND: u16 = 7;
pub const OP_RECV: u16 = 8;
pub const OP_CLOSE: u16 = 9;
pub const SOCKETS_MAGIC: u32 = 0x4E53_4B54;
pub const SERVICE_DNS: &[u8] = b"net.dns";
pub const SERVICE_NYM: &[u8] = b"net.nym";
pub const SERVICE_SOCKETS: &[u8] = b"net.sockets";
pub const SOCKET_FAMILY_IP4: u16 = 4;
pub const SOCKET_KIND_STREAM: u16 = 1;
