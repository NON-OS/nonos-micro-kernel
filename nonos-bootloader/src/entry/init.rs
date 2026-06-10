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

pub fn init_boot_services(st: &mut SystemTable<Boot>) {
    let _ = st.stdout().reset(false);
    let _ = st.stdout().output_string(uefi::cstr16!("[BOOT] NONOS Bootloader v1.0\r\n"));
    if uefi_services::init(st).is_err() {
        let _ = st.stdout().output_string(uefi::cstr16!("[FATAL] UEFI init failed\r\n"));
        halt();
    }
    let _ = st.boot_services().set_watchdog_timer(0, 0x10000, None);
}

fn halt() -> ! {
    loop {
        core::hint::spin_loop();
    }
}
