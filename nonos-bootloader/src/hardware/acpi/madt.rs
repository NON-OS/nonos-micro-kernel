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

use super::find::find_madt_table;
use super::parse::parse_madt_cpu_count;

pub fn get_cpu_count_from_acpi(rsdp_address: u64) -> usize {
    let madt_addr = find_madt_table(rsdp_address);
    if madt_addr == 0 {
        return 1;
    }
    parse_madt_cpu_count(madt_addr)
}
