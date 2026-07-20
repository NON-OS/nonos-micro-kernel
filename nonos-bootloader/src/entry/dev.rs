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

#[cfg(feature = "dev-mode")]
use nonos_boot::menu::check_dev_key_held;

#[cfg(feature = "dev-mode")]
pub fn dev_override(st: &mut SystemTable<Boot>) -> bool {
    if !check_dev_key_held(st.boot_services()) {
        return false;
    }
    if secure_boot_enabled(st) {
        let _ = st.stdout().output_string(uefi::cstr16!(
            "[SECURITY] F12 dev mode blocked: Secure Boot is enabled\r\n"
        ));
        return false;
    }
    let _ = st.stdout().output_string(uefi::cstr16!(
        "[WARN] F12 pressed - development mode (Secure Boot disabled)\r\n"
    ));
    true
}

#[cfg(not(feature = "dev-mode"))]
pub fn dev_override(_: &mut SystemTable<Boot>) -> bool {
    false
}

#[cfg(feature = "dev-mode")]
fn secure_boot_enabled(st: &mut SystemTable<Boot>) -> bool {
    // The SecureBoot global variable is a single byte; uefi 0.23 writes it into a
    // caller-provided buffer and returns the filled slice with its attributes.
    let mut buf = [0u8; 1];
    st.runtime_services()
        .get_variable(
            uefi::cstr16!("SecureBoot"),
            &uefi::table::runtime::VariableVendor::GLOBAL_VARIABLE,
            &mut buf,
        )
        .map(|(data, _)| data.first().copied().unwrap_or(0) == 1)
        .unwrap_or(false)
}
