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

pub unsafe fn vendor_string() -> String {
    let (_, ebx, ecx, edx) = cpuid(0);
    let mut bytes = [0u8; 12];
    bytes[0..4].copy_from_slice(&ebx.to_le_bytes());
    bytes[4..8].copy_from_slice(&edx.to_le_bytes());
    bytes[8..12].copy_from_slice(&ecx.to_le_bytes());
    String::from_utf8_lossy(&bytes).into_owned()
}
