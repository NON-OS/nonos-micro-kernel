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

use super::tables::AcpiSdtHeader;

const LAPIC_ENTRY_TYPE: u8 = 0;
const LAPIC_FLAGS_ENABLED: u32 = 1;

pub fn parse_madt_cpu_count(madt_addr: u64) -> usize {
    unsafe {
        let hdr = &*(madt_addr as *const AcpiSdtHeader);
        let mut cpu_count = 0usize;
        let mut offset = 44usize;
        while offset + 2 <= hdr.length as usize {
            let entry_type = *((madt_addr + offset as u64) as *const u8);
            let entry_len = *((madt_addr + offset as u64 + 1) as *const u8) as usize;
            if entry_len < 2 {
                break;
            }
            if entry_type == LAPIC_ENTRY_TYPE && entry_len >= 8 {
                let flags = *((madt_addr + offset as u64 + 4) as *const u32);
                if flags & LAPIC_FLAGS_ENABLED != 0 {
                    cpu_count += 1;
                }
            }
            offset += entry_len;
        }
        if cpu_count == 0 {
            1
        } else {
            cpu_count
        }
    }
}
