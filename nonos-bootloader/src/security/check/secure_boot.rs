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

extern crate alloc;

use crate::log::logger::{log_error, log_info};
use alloc::format;
use uefi::cstr16;
use uefi::prelude::*;

pub fn check_secure_boot(st: &mut SystemTable<Boot>) -> bool {
    let rt = st.runtime_services();
    let mut buf = [0u8; 1];
    match rt.get_variable(
        cstr16!("SecureBoot"),
        &uefi::table::runtime::VariableVendor::GLOBAL_VARIABLE,
        &mut buf,
    ) {
        Ok(_) => {
            let on = buf[0] == 1;
            log_info("security", if on { "SecureBoot ENABLED" } else { "SecureBoot DISABLED" });
            on
        }
        Err(e) => {
            log_error("security", &format!("Cannot read SecureBoot: {:?}", e.status()));
            false
        }
    }
}

pub fn check_platform_key(st: &mut SystemTable<Boot>) -> bool {
    let rt = st.runtime_services();
    let mut buf = [0u8; 2048];
    match rt.get_variable(
        cstr16!("PK"),
        &uefi::table::runtime::VariableVendor::GLOBAL_VARIABLE,
        &mut buf,
    ) {
        Ok(_) => {
            log_info("security", "Platform Key present");
            buf.iter().any(|&b| b != 0)
        }
        Err(e) => {
            log_error("security", &format!("Platform Key missing: {:?}", e.status()));
            false
        }
    }
}
