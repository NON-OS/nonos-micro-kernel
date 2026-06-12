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

use super::types::CpuId;
use alloc::string::String;

#[cfg(target_arch = "x86_64")]
pub fn read() -> CpuId {
    use super::brand::brand_string;
    use super::cpuid::cpuid;
    use super::vendor::vendor_string;
    unsafe {
        let (max_ext, _, _, _) = cpuid(0x8000_0000);
        let brand = if max_ext >= 0x8000_0004 { brand_string() } else { String::new() };
        let (_, ebx1, _, _) = cpuid(1);
        CpuId { brand: brand.trim().into(), vendor: vendor_string(), logical: (ebx1 >> 16) & 0xFF }
    }
}

#[cfg(not(target_arch = "x86_64"))]
pub fn read() -> CpuId {
    CpuId { brand: String::new(), vendor: String::new(), logical: 0 }
}
