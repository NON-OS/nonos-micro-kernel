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

//! Request bytes as the specification lays them out, for comparing against
//! what the part saw.

use crate::device::cmd::hdr::{Hdr, HDR_LEN};
use crate::device::cmd::transfer_to_host_2d::Rect;

pub const RECT: Rect = Rect { x: 8, y: 16, width: 640, height: 480 };

/// A bare header of `type_`, the whole of a request with no body.
pub fn request(type_: u32) -> [u8; HDR_LEN] {
    let mut req = [0u8; HDR_LEN];
    Hdr::new(type_, 0).write(&mut req);
    req
}

/// A header with no fence flag and no context, then `body` as little-endian
/// words.
pub fn expected(type_: u32, fence: u64, body: &[u32]) -> Vec<u8> {
    let mut v = Vec::new();
    v.extend_from_slice(&type_.to_le_bytes());
    v.extend_from_slice(&0u32.to_le_bytes());
    v.extend_from_slice(&fence.to_le_bytes());
    v.extend_from_slice(&[0u8; 8]);
    for w in body {
        v.extend_from_slice(&w.to_le_bytes());
    }
    v
}
