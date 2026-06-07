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

use crate::syscall::{call_raw, N_MK_ATTEST_STATUS};

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AttestStatus {
    pub zk_verified: u8,
    pub kernel_sig_ok: u8,
    pub secure_boot: u8,
    pub zk_attestation_ok: u8,
    pub kernel_blake3: [u8; 32],
    pub program_hash: [u8; 32],
}

pub extern "C" fn mk_attest_status(out: *mut AttestStatus) -> i64 {
    call_raw(N_MK_ATTEST_STATUS, [out as u64, 0, 0, 0, 0, 0])
}
