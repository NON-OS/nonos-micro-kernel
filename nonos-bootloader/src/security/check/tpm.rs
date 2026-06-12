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

use crate::log::logger::{log_debug, log_info};
use crate::security::{extend_pcr_measurement, PCR_BOOTLOADER};
use uefi::prelude::*;

pub fn check_measured_boot(st: &mut SystemTable<Boot>) -> bool {
    let test_data = b"NONOS:TPM:PROBE:v1";
    if extend_pcr_measurement(st, PCR_BOOTLOADER, test_data) {
        log_info("security", "TPM 2.0 measured boot available");
        true
    } else {
        log_debug("security", "TPM 2.0 not available");
        false
    }
}
