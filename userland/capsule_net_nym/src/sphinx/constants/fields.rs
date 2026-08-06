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

/// k in the Sphinx paper, in bytes.
pub const SECURITY_PARAMETER: usize = 16;

/// r in the Sphinx paper: the longest route a header can describe.
pub const MAX_PATH_LENGTH: usize = 5;

pub const NODE_ADDRESS_LENGTH: usize = 2 * SECURITY_PARAMETER;
pub const DESTINATION_ADDRESS_LENGTH: usize = 2 * SECURITY_PARAMETER;
pub const IDENTIFIER_LENGTH: usize = SECURITY_PARAMETER;
pub const FLAG_LENGTH: usize = 1;
pub const DELAY_LENGTH: usize = 8;
/// Major, minor, patch, one byte each.
pub const VERSION_LENGTH: usize = 3;

pub const HEADER_INTEGRITY_MAC_SIZE: usize = SECURITY_PARAMETER;
pub const INTEGRITY_MAC_KEY_SIZE: usize = SECURITY_PARAMETER;
pub const BLINDING_FACTOR_SIZE: usize = 2 * SECURITY_PARAMETER;
pub const REPLAY_TAG_SIZE: usize = 2 * SECURITY_PARAMETER;
pub const STREAM_CIPHER_KEY_SIZE: usize = SECURITY_PARAMETER;
pub const PAYLOAD_KEY_SEED_SIZE: usize = SECURITY_PARAMETER;

/// Fixed by the LIONESS instantiation: two ChaCha20 keys of 32 and two
/// BLAKE2b MAC keys of 64.
pub const PAYLOAD_KEY_SIZE: usize = 192;
