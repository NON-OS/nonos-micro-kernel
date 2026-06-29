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

pub const OP_X25519_PUBLIC: u16 = 14;
pub const OP_X25519_SHARED: u16 = 15;
pub const OP_HMAC_SHA256: u16 = 16;
pub const OP_HKDF_SHA256: u16 = 17;
pub const OP_P256_ECDSA_VERIFY: u16 = 18;
pub const OP_P384_ECDSA_VERIFY: u16 = 19;
pub const OP_SHA384_HASH: u16 = 20;
pub const OP_RSA_VERIFY: u16 = 21;

pub const X25519_KEY_BYTES: usize = 32;
pub const P256_VERIFY_BYTES: usize = 65 + 64 + 32;
pub const P384_VERIFY_BYTES: usize = 97 + 96 + 48;
pub const HMAC_KEY_MAX: usize = 256;
pub const HKDF_PART_MAX: usize = 256;
pub const HKDF_OUT_MAX: usize = 512;
