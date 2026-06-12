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

use super::super::binding::{verify_commitment_binding, verify_kernel_in_proof};
use super::debug::log_hex;
use super::failure::binding_failure;
use crate::log::logger::{log_error, log_info};
use crate::zk::binding::compute_capsule_commitment;
use crate::zk::{parse_zk_proof, BootAttestationResult};
use uefi::prelude::*;

pub fn enforce_zk_binding(
    st: &mut SystemTable<Boot>,
    r: &BootAttestationResult,
    kb: &[u8],
    kh: &[u8; 32],
    gop: bool,
) {
    let (proof_block, _) = match parse_zk_proof(kb) {
        Ok(pb) => pb,
        Err(e) => {
            log_error("zk_bind", "Failed to parse proof");
            log_error("zk_bind", e);
            binding_failure(st, gop, e);
        }
    };
    if let Err(e) = verify_kernel_in_proof(r, kh, &proof_block) {
        log_error("zk_bind", e);
        binding_failure(st, gop, e);
    }
    let expected = compute_capsule_commitment(
        kh,
        &proof_block.boot_nonce,
        &proof_block.machine_id,
        &proof_block.program_hash,
    );
    log_info("zk_bind", "comparing commitments");
    log_hex("stored", &r.capsule_commitment[..8]);
    log_hex("expect", &expected[..8]);
    log_hex("kh_act", &kh[..8]);
    log_hex("kh_blk", &proof_block.kernel_hash[..8]);
    if let Err(e) = verify_commitment_binding(&r.capsule_commitment, &expected) {
        log_error("zk_bind", e);
        binding_failure(st, gop, e);
    }
}
