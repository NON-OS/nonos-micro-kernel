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

use super::pcr::extend_pcr_measurement;
use crate::log::logger::log_debug;
use crate::security::tpm_types::{PCR_BOOTLOADER, PCR_CAPSULE, PCR_KERNEL};

pub fn measure_boot_components(
    st: &mut SystemTable<Boot>,
    bl: &[u8],
    kern: &[u8],
    caps: &[u8],
) -> bool {
    let mut ok = true;
    if !extend_pcr_measurement(st, PCR_BOOTLOADER, bl) {
        log_debug("security", "bootloader not extended");
        ok = false;
    }
    if !extend_pcr_measurement(st, PCR_KERNEL, kern) {
        log_debug("security", "kernel not extended");
        ok = false;
    }
    if !extend_pcr_measurement(st, PCR_CAPSULE, caps) {
        log_debug("security", "capsule not extended");
        ok = false;
    }
    ok
}
