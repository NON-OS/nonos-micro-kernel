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

use uefi::prelude::*;
use uefi::table::cfg::{SMBIOS3_GUID, SMBIOS_GUID};

/// Find SMBIOS entry point from UEFI config table. Prefers SMBIOS 3.0. Returns 0 if not found.
pub fn get_smbios_entry(st: &SystemTable<Boot>) -> u64 {
    for entry in st.config_table() {
        if entry.guid == SMBIOS3_GUID {
            return entry.address as u64;
        }
    }
    for entry in st.config_table() {
        if entry.guid == SMBIOS_GUID {
            return entry.address as u64;
        }
    }
    0
}
