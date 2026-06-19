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

use nonos_boot::boot::exit_to_shell;
use nonos_boot::menu::{MenuAction, SecurityMode};
use uefi::prelude::*;
use uefi::table::runtime::ResetType;

pub fn resolve_action(
    st: &mut SystemTable<Boot>,
    action: MenuAction,
) -> Result<SecurityMode, Status> {
    match action {
        MenuAction::Boot(mode) => Ok(mode),
        MenuAction::Timeout | MenuAction::Continue => Ok(SecurityMode::Standard),
        MenuAction::SafeMode => {
            let _ = st.stdout().output_string(uefi::cstr16!("[BOOT] Safe Mode\r\n"));
            Ok(SecurityMode::SafeMode)
        }
        MenuAction::NetworkIsolated => {
            let _ = st.stdout().output_string(uefi::cstr16!("[BOOT] Air-Gapped Mode\r\n"));
            Ok(SecurityMode::NetworkIsolated)
        }
        MenuAction::Recovery => {
            let _ = st.stdout().output_string(uefi::cstr16!("[BOOT] Recovery Mode\r\n"));
            Ok(SecurityMode::Recovery)
        }
        MenuAction::UefiShell => {
            let _ = st.stdout().output_string(uefi::cstr16!("[BOOT] UEFI Shell\r\n"));
            Err(exit_to_shell(st))
        }
        MenuAction::Shutdown => {
            let _ = st.stdout().output_string(uefi::cstr16!("[BOOT] Shutdown\r\n"));
            st.runtime_services().reset(ResetType::SHUTDOWN, Status::SUCCESS, None);
        }
        MenuAction::Diagnostics => {
            let _ = st.stdout().output_string(uefi::cstr16!("[DIAG] Hardware Diagnostics\r\n"));
            Ok(SecurityMode::SafeMode)
        }
        MenuAction::SecurityStatus => {
            let _ = st.stdout().output_string(uefi::cstr16!("[INFO] Security Status\r\n"));
            Ok(SecurityMode::Standard)
        }
    }
}
