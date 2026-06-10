// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::cpuid::cpuid;
use alloc::string::String;

pub unsafe fn brand_string() -> String {
    let mut bytes = [0u8; 48];
    for (i, leaf) in [0x8000_0002u32, 0x8000_0003, 0x8000_0004].iter().enumerate() {
        let (a, b, c, d) = cpuid(*leaf);
        let off = i * 16;
        bytes[off..off + 4].copy_from_slice(&a.to_le_bytes());
        bytes[off + 4..off + 8].copy_from_slice(&b.to_le_bytes());
        bytes[off + 8..off + 12].copy_from_slice(&c.to_le_bytes());
        bytes[off + 12..off + 16].copy_from_slice(&d.to_le_bytes());
    }
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(48);
    String::from_utf8_lossy(&bytes[..end]).into_owned()
}
