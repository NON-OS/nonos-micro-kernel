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

use crate::log::logger::log_warn;
use uefi::cstr16;
use uefi::prelude::*;

pub fn detect_secure_boot_bypass(system_table: &mut SystemTable<Boot>) -> bool {
    let rt = system_table.runtime_services();
    let mut setup_mode = [0u8; 1];
    if let Ok(_) = rt.get_variable(
        cstr16!("SetupMode"),
        &uefi::table::runtime::VariableVendor::GLOBAL_VARIABLE,
        &mut setup_mode,
    ) {
        if setup_mode[0] == 1 {
            log_warn("enforce", "UEFI in SetupMode");
            return true;
        }
    }
    let mut audit_mode = [0u8; 1];
    if let Ok(_) = rt.get_variable(
        cstr16!("AuditMode"),
        &uefi::table::runtime::VariableVendor::GLOBAL_VARIABLE,
        &mut audit_mode,
    ) {
        if audit_mode[0] == 1 {
            log_warn("enforce", "UEFI in AuditMode");
            return true;
        }
    }
    false
}
