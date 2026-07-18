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

//! How the boot proves the kernel it is about to jump to.
//!
//! In a transparent-STARK build the kernel carries its own money-grade
//! self-attestation. It is verified earlier in kernel_verify and the outcome is
//! recorded on the crypto result; here that proof simply becomes the boot
//! attestation, and no curve boot-binding runs. There is deliberately zero
//! legacy ZK on this path.
//!
//! In a legacy build the curve boot-binding still applies unchanged.

use crate::display::BootCryptoState;
use crate::kernel_verify::CryptoVerifyResult;
use crate::menu::SecurityMode;
use crate::zk::BootAttestationResult;
use uefi::prelude::*;

/// Attest the loaded kernel and return the boot attestation result carried into
/// the handoff. The signature takes both the raw image and the already-computed
/// crypto result so either build profile has what it needs; the STARK profile
/// only reads the result, the curve profile drives the full binding.
pub fn attest_kernel(
    st: &mut SystemTable<Boot>,
    data: &[u8],
    crypto: &CryptoVerifyResult,
    crypto_state: &mut BootCryptoState,
    gop: bool,
    tpm: bool,
    mode: SecurityMode,
) -> BootAttestationResult {
    #[cfg(feature = "stark-kernel-attest")]
    {
        // The curve boot-binding inputs are unused when the kernel self-attests.
        let _ = (data, crypto_state, tpm);
        stark_kernel_gate(st, crypto, gop, mode)
    }
    #[cfg(not(feature = "stark-kernel-attest"))]
    {
        super::run_zk_attestation(st, data, &crypto.kernel_hash_full, crypto_state, gop, tpm, mode)
    }
}

/// The transparent-STARK kernel gate. kernel_verify already checked the kernel's
/// self-attestation against the enrolled boot root; act on that verdict.
#[cfg(feature = "stark-kernel-attest")]
fn stark_kernel_gate(
    st: &mut SystemTable<Boot>,
    crypto: &CryptoVerifyResult,
    gop: bool,
    mode: SecurityMode,
) -> BootAttestationResult {
    use crate::boot::uefi::TOTAL_BOOT_STAGES;
    use crate::boot::util::fatal_reset;
    use crate::display::{
        draw_boot_progress, show_error_screen, update_stage, StageStatus, STAGE_ZK_VERIFY,
    };
    use crate::log::logger::{log_error, log_info, log_warn};

    if crypto.stark_gate_satisfied() {
        // The kernel proved its own measurement under the enrolled boot root. That
        // proof is the boot attestation; the kernel hash is the measurement it
        // commits to, so bind the result to it on both public inputs.
        log_info("zk", "kernel STARK self-attestation is the boot proof");
        update_stage(STAGE_ZK_VERIFY, StageStatus::Success);
        draw_boot_progress(8, TOTAL_BOOT_STAGES);
        return BootAttestationResult::verified(crypto.kernel_hash_full, crypto.kernel_hash_full);
    }

    if mode == SecurityMode::Development {
        // Dev builds may run an un-attested kernel on signature trust alone, the
        // same latitude the curve path allowed, but say so loudly.
        log_warn("zk", "kernel STARK self-attestation absent - dev mode, skipping");
        update_stage(STAGE_ZK_VERIFY, StageStatus::Success);
        draw_boot_progress(8, TOTAL_BOOT_STAGES);
        return BootAttestationResult::default();
    }

    // Production STARK build: a kernel that cannot prove itself does not boot.
    log_error("zk", "kernel STARK self-attestation missing or invalid");
    update_stage(STAGE_ZK_VERIFY, StageStatus::Failed);
    if gop {
        show_error_screen(b"kernel attestation required");
    }
    fatal_reset(st, "kernel STARK self-attestation missing")
}
