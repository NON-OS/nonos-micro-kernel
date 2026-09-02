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

use super::disk::Disk;
use super::value::Metric;

pub const HDR_LEN: usize = 20;
pub const BODY_LEN: usize = 20;
pub const REPLY_LEN: usize = HDR_LEN + BODY_LEN;

const STATUS_OK: i32 = 0;

/// A short reply, or one carrying a failure status, is a store that could not
/// be measured rather than a store holding nothing.
pub fn decode_usage(rx: &[u8]) -> Disk {
    if rx.len() < REPLY_LEN || status(rx) != STATUS_OK {
        return Disk::UNKNOWN;
    }
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&rx[HDR_LEN + 8..HDR_LEN + 16]);
    Disk { used_kb: Metric::Known(u64::from_le_bytes(bytes) / 1024), ..Disk::UNKNOWN }
}

fn status(rx: &[u8]) -> i32 {
    let mut raw = [0u8; 4];
    raw.copy_from_slice(&rx[HDR_LEN..HDR_LEN + 4]);
    i32::from_le_bytes(raw)
}
