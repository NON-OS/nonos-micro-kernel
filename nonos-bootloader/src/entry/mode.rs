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

use super::action::resolve_action;
use nonos_boot::fwui;
use nonos_boot::hardware::HardwareInfo;
use nonos_boot::menu::SecurityMode;
use nonos_boot::security::SecurityContext;
use uefi::prelude::*;

pub fn select_security_mode(
    st: &mut SystemTable<Boot>,
    dev_override: bool,
    security: &SecurityContext,
    hw: &HardwareInfo,
) -> Result<SecurityMode, Status> {
    if dev_override {
        let _ = st.stdout().output_string(uefi::cstr16!("[WARN] DEV MODE - SECURITY BYPASSED\r\n"));
        return Ok(SecurityMode::Development);
    }
    let action = fwui::run(st, security, hw);
    resolve_action(st, action)
}
