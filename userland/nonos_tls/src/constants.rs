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

pub const TLS_HANDSHAKE: u8 = 22;
pub const HS_CLIENT_HELLO: u8 = 1;
pub const LEGACY_RECORD_VERSION: u16 = 0x0301;
pub const LEGACY_HANDSHAKE_VERSION: u16 = 0x0303;
pub const TLS13: u16 = 0x0304;
pub const SUITE_CHACHA20_SHA256: u16 = 0x1303;
pub const SUITE_AES128_GCM_SHA256: u16 = 0x1301;
pub const GROUP_X25519: u16 = 0x001d;
pub const EXT_SERVER_NAME: u16 = 0;
pub const EXT_SUPPORTED_GROUPS: u16 = 10;
pub const EXT_SIGNATURE_ALGORITHMS: u16 = 13;
pub const EXT_SUPPORTED_VERSIONS: u16 = 43;
pub const EXT_KEY_SHARE: u16 = 51;
