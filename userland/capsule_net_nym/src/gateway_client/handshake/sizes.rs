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

pub const IDENTITY_BYTES: usize = 32;
pub const SIGNATURE_BYTES: usize = 64;
pub const EPHEMERAL_BYTES: usize = 32;
/// KDF_SALT_LENGTH in the reference.
pub const SALT_BYTES: usize = 16;
pub const TAG_BYTES: usize = 16;
pub const NONCE_BYTES: usize = 12;

/// identity || ephemeral || salt
pub const INIT_BYTES: usize = IDENTITY_BYTES + EPHEMERAL_BYTES + SALT_BYTES;
/// sealed signature || nonce
pub const MATERIAL_BYTES: usize = SIGNATURE_BYTES + TAG_BYTES + NONCE_BYTES;
/// the gateway prepends its own ephemeral key
pub const GATEWAY_MATERIAL_BYTES: usize = EPHEMERAL_BYTES + MATERIAL_BYTES;

const _: () = assert!(INIT_BYTES == 80);
const _: () = assert!(MATERIAL_BYTES == 92);
const _: () = assert!(GATEWAY_MATERIAL_BYTES == 124);
