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

use nonos_boot::boot::prepare::HandoffParams;
use nonos_boot::boot::{
    attest_kernel, commit_rollback, run_crypto_verification, run_elf_parse, run_handoff_prepare,
    run_kernel_load,
};
use nonos_boot::kernel_verify::CryptoVerifyResult;
use nonos_boot::menu::SecurityMode;
use nonos_boot::security::SecurityContext;
use nonos_boot::zk::BootAttestationResult;
use uefi::prelude::*;

pub fn run_verified_boot(
    mut st: SystemTable<Boot>,
    gop: bool,
    security: SecurityContext,
    mode: SecurityMode,
) -> ! {
    let kernel_data = run_kernel_load(&mut st, gop);
    let (crypto_result, mut crypto_state) =
        run_crypto_verification(&mut st, &kernel_data, gop, mode);
    let zk_result = attest_kernel(
        &mut st,
        &kernel_data,
        &crypto_result,
        &mut crypto_state,
        gop,
        security.measured_boot_active,
        mode,
    );
    let kernel_image = run_elf_parse(&mut st, &kernel_data, &crypto_result, gop);
    commit_rollback(&mut st, &kernel_data, mode, gop);
    let params = handoff_params(&security, &crypto_result, zk_result);
    run_handoff_prepare(st, &kernel_image, params, gop);
}

fn handoff_params(
    security: &SecurityContext,
    crypto: &CryptoVerifyResult,
    zk_result: BootAttestationResult,
) -> HandoffParams {
    HandoffParams {
        signature_valid: crypto.signature_valid,
        secure_boot: security.secure_boot_enabled,
        kernel_hash: crypto.kernel_hash_full,
        zk_result,
        tpm_measured: security.measured_boot_active,
    }
}
