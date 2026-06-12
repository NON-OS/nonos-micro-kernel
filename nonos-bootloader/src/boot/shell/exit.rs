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

use super::find::find_shell;
use super::launch::launch_shell;
use uefi::prelude::*;

pub fn exit_to_shell(st: &mut SystemTable<Boot>) -> Status {
    let _ = st.stdout().output_string(uefi::cstr16!("  [SHELL] Searching for UEFI Shell...\r\n"));
    let shell_path = find_shell(st.boot_services());
    if let Some(path) = shell_path {
        let _ = st.stdout().output_string(uefi::cstr16!("  [SHELL] Found: "));
        let _ = st.stdout().output_string(path);
        let _ = st.stdout().output_string(uefi::cstr16!("\r\n"));
        return launch_shell(st.boot_services(), path);
    }
    let _ = st
        .stdout()
        .output_string(uefi::cstr16!("  [SHELL] Not found. Returning to firmware menu.\r\n"));
    st.boot_services().stall(2_000_000);
    Status::NOT_FOUND
}
