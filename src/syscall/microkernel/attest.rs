// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
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

//! `MkAttestStatus` exposes the read-only boot-chain attestation result the
//! bootloader recorded in the handoff (`Measurements` + `ZkAttestation`) so a
//! capsule can surface it. It does not run or re-verify any proof; the ZK
//! verifier surface is separate and owned elsewhere.

use super::errnos::ERRNO_FAULT;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AttestStatusAbi {
    pub zk_verified: u8,
    pub kernel_sig_ok: u8,
    pub secure_boot: u8,
    pub zk_attestation_ok: u8,
    pub kernel_blake3: [u8; 32],
    pub program_hash: [u8; 32],
}

pub fn sys_attest_status(out_ptr: u64) -> i64 {
    let Some(h) = crate::boot::handoff::get_handoff() else {
        return ERRNO_FAULT;
    };
    let abi = AttestStatusAbi {
        zk_verified: h.zk.verified,
        kernel_sig_ok: h.meas.kernel_sig_ok,
        secure_boot: h.meas.secure_boot,
        zk_attestation_ok: h.meas.zk_attestation_ok,
        kernel_blake3: h.meas.kernel_blake3,
        program_hash: h.zk.program_hash,
    };
    if crate::usercopy::validate_user_write(out_ptr, core::mem::size_of::<AttestStatusAbi>()).is_err()
    {
        return ERRNO_FAULT;
    }
    match crate::usercopy::write_user_value(out_ptr, &abi) {
        Ok(()) => 0,
        Err(_) => ERRNO_FAULT,
    }
}
