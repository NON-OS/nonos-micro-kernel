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

use uefi::{cstr16, CStr16};

use crate::config::types::{GraphicsMode, NetworkPolicy, SecurityPolicy};

pub const MAX_BOOT_TIMEOUT: u32 = 300;

pub fn load_security_policy(rt: &uefi::table::runtime::RuntimeServices) -> Option<SecurityPolicy> {
    let mut buffer = [0u8; 4];
    let var_name = cstr16!("NonosSecurityPolicy");

    match rt.get_variable(
        var_name,
        &uefi::table::runtime::VariableVendor::GLOBAL_VARIABLE,
        &mut buffer,
    ) {
        Ok(_) => SecurityPolicy::from_u8(buffer[0]),
        Err(_) => None,
    }
}

pub fn load_network_policy(rt: &uefi::table::runtime::RuntimeServices) -> Option<NetworkPolicy> {
    let mut buffer = [0u8; 4];
    let var_name = cstr16!("NonosNetworkPolicy");

    match rt.get_variable(
        var_name,
        &uefi::table::runtime::VariableVendor::GLOBAL_VARIABLE,
        &mut buffer,
    ) {
        Ok(_) => NetworkPolicy::from_u8(buffer[0]),
        Err(_) => None,
    }
}

pub fn load_boot_timeout(rt: &uefi::table::runtime::RuntimeServices) -> Option<u32> {
    let mut buffer = [0u8; 4];
    let var_name = cstr16!("NonosBootTimeout");

    match rt.get_variable(
        var_name,
        &uefi::table::runtime::VariableVendor::GLOBAL_VARIABLE,
        &mut buffer,
    ) {
        Ok(_) => {
            let timeout = u32::from_le_bytes(buffer);
            if timeout <= MAX_BOOT_TIMEOUT {
                Some(timeout)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

pub fn load_graphics_mode(rt: &uefi::table::runtime::RuntimeServices) -> Option<GraphicsMode> {
    let mut buffer = [0u8; 4];
    let var_name = cstr16!("NonosGraphicsMode");

    match rt.get_variable(
        var_name,
        &uefi::table::runtime::VariableVendor::GLOBAL_VARIABLE,
        &mut buffer,
    ) {
        Ok(_) => GraphicsMode::from_u8(buffer[0]),
        Err(_) => None,
    }
}

pub fn load_bool_variable(rt: &uefi::table::runtime::RuntimeServices, name: &CStr16) -> bool {
    let mut buffer = [0u8; 1];

    match rt.get_variable(name, &uefi::table::runtime::VariableVendor::GLOBAL_VARIABLE, &mut buffer)
    {
        Ok(_) => buffer[0] != 0,
        Err(_) => false,
    }
}
