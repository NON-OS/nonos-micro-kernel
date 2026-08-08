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

use super::super::chacha20::KEY_BYTES as STREAM_KEY_BYTES;

/// Blake2bMac<U32>: the digest width, and so the left-half width.
pub const MAC_BYTES: usize = 32;
pub const MAC_KEY_BYTES: usize = 64;
/// 2 * (32 + 64). Equals Sphinx's PAYLOAD_KEY_SIZE, which is where that
/// otherwise arbitrary 192 comes from.
pub const KEY_BYTES: usize = 2 * (STREAM_KEY_BYTES + MAC_KEY_BYTES);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BlockTooShort;

pub struct Lioness {
    pub(super) k1: [u8; STREAM_KEY_BYTES],
    pub(super) k2: [u8; MAC_KEY_BYTES],
    pub(super) k3: [u8; STREAM_KEY_BYTES],
    pub(super) k4: [u8; MAC_KEY_BYTES],
}
