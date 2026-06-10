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

use crate::log::logger::{log_info, log_warn};
use uefi::prelude::*;

#[cfg(target_arch = "x86_64")]
use crate::security::cpuid::cpu_rng_supported;

pub fn check_hardware_rng(st: &mut SystemTable<Boot>) -> bool {
    let bs = st.boot_services();
    if let Ok(handles) = bs.find_handles::<uefi::proto::rng::Rng>() {
        if !handles.is_empty() {
            log_info("rng", "EFI RNG protocol detected");
            return true;
        }
    }
    #[cfg(target_arch = "x86_64")]
    if cpu_rng_supported() {
        log_info("rng", "CPU RDRAND/RDSEED available");
        return true;
    }
    log_warn("rng", "No hardware RNG found");
    false
}
