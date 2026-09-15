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

use alloc::string::String;
use alloc::vec::Vec;

pub const SIDECAR_MAGIC: u32 = 0x4E53_4443;
pub const SIDECAR_VERSION: u32 = 1;

// Serialize key/value records sorted by key, so the same map always produces
// the same bytes and a diff of the stored blob is meaningful.
pub fn encode(records: &[(String, String)]) -> Vec<u8> {
    let mut sorted: Vec<&(String, String)> = records.iter().collect();
    sorted.sort_by(|a, b| a.0.cmp(&b.0));
    let mut out = Vec::new();
    out.extend_from_slice(&SIDECAR_MAGIC.to_le_bytes());
    out.extend_from_slice(&SIDECAR_VERSION.to_le_bytes());
    out.extend_from_slice(&(sorted.len() as u32).to_le_bytes());
    for (key, val) in sorted {
        out.extend_from_slice(&(key.len() as u32).to_le_bytes());
        out.extend_from_slice(key.as_bytes());
        out.extend_from_slice(&(val.len() as u32).to_le_bytes());
        out.extend_from_slice(val.as_bytes());
    }
    out
}
