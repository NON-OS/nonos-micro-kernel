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

pub const E_OK: u16 = 0;
pub const E_BAD_MAGIC: u16 = 1;
pub const E_BAD_VERSION: u16 = 2;
pub const E_BAD_OP: u16 = 3;
pub const E_BAD_LEN: u16 = 4;
pub const E_NO_TCP: u16 = 5;
pub const E_NO_GATEWAY: u16 = 6;
pub const E_TABLE_FULL: u16 = 7;
pub const E_NO_SESSION: u16 = 8;
pub const E_CRYPTO: u16 = 9;
pub const E_RX_EMPTY: u16 = 10;
pub const E_NO_TOPOLOGY: u16 = 11;
pub const E_NO_CREDENTIAL: u16 = 12;
pub const E_NO_ROUTE: u16 = 13;
pub const E_CREDENTIAL_EXPIRED: u16 = 14;
pub const E_GATEWAY_PROTO: u16 = 15;
pub const E_TOPOLOGY_AUTH: u16 = 16;
pub const E_TOPOLOGY_STALE: u16 = 17;
pub const E_AUTHORITY_MISSING: u16 = 18;
pub const E_AUTHORITY_UNTRUSTED: u16 = 19;
pub const E_DIRECTORY_PROTO: u16 = 20;
pub const E_DIRECTORY_SOURCE: u16 = 21;
pub const E_TOPOLOGY_EXPIRED: u16 = 22;
pub const E_PERM: u16 = 23;

/// A second session tried to bind a destination while one already holds it.
pub const E_BUSY: u16 = 24;
