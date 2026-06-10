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
use crate::security::{extend_pcr_measurement, PCR_CAPSULE, PCR_KERNEL};
use uefi::prelude::*;

pub fn extend_boot_measurements(
    system_table: &mut SystemTable<Boot>,
    kernel_hash: &[u8; 32],
    signature: &[u8; 64],
    zk_proof_hash: &[u8; 32],
) -> bool {
    let mut composite = [0u8; 128];
    composite[0..32].copy_from_slice(kernel_hash);
    composite[32..96].copy_from_slice(signature);
    composite[96..128].copy_from_slice(zk_proof_hash);
    let extended = extend_pcr_measurement(system_table, PCR_KERNEL, &composite);
    if extended {
        log_info("enforce", "measurements extended to PCR9");
    } else {
        log_warn("enforce", "TPM not available");
    }
    let _ = extend_pcr_measurement(system_table, PCR_CAPSULE, zk_proof_hash);
    extended
}
