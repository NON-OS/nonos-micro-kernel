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

use super::guid::VENDOR;
use super::state::Settings;
use uefi::cstr16;
use uefi::prelude::*;
use uefi::table::runtime::VariableAttributes;

pub fn save(st: &SystemTable<Boot>, s: &Settings) {
    let data = [s.default_mode, s.timeout_s, s.enforce_sb as u8];
    let attrs = VariableAttributes::NON_VOLATILE
        | VariableAttributes::BOOTSERVICE_ACCESS
        | VariableAttributes::RUNTIME_ACCESS;
    let _ = st.runtime_services().set_variable(cstr16!("NonosSetup"), &VENDOR, attrs, &data);
}
