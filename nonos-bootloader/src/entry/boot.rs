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

use super::dev::dev_override;
use super::init::init_boot_services;
use super::mode::select_security_mode;
use super::pipeline::run_verified_boot;
use nonos_boot::boot::{
    initialize_zk_replay_protection, run_hardware_discovery, run_security_checks, run_uefi_init,
};
use nonos_boot::display::{draw_status_line, init_boot_screen};
use uefi::prelude::*;

pub fn boot_entry(_handle: Handle, mut st: SystemTable<Boot>) -> Status {
    init_boot_services(&mut st);
    let uefi_result = run_uefi_init(&mut st);
    let gop = uefi_result.gop_available;
    let dev_mode = dev_override(&mut st);
    let security = run_security_checks(&mut st, gop);
    let hw = run_hardware_discovery(&mut st, gop);
    let security_mode = match select_security_mode(&mut st, dev_mode, &security, &hw) {
        Ok(mode) => mode,
        Err(status) => return status,
    };
    if gop {
        init_boot_screen();
        draw_status_line(security.secure_boot_enabled, security.measured_boot_active, false);
    }
    initialize_zk_replay_protection(&st);
    run_verified_boot(st, gop, security, security_mode);
}
