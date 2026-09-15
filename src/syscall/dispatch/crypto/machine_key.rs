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

//! `CryptoMachineKey`: a 32-byte key bound to this TPM and this boot state.
//!
//! The caller names the key with a label and gets the same bytes back every
//! boot on this machine, and different bytes on any other machine or under any
//! other kernel. Nothing is stored anywhere to make that so. The volume store
//! wraps its volume key under one of these; the wallet wraps its seed under
//! another. Neither could persist anything across a reboot before this.

use crate::capabilities::Capability;
use crate::security::tpm::error::TpmError;
use crate::security::tpm::machine_key::{derive, KeyError, LABEL_MAX};
use crate::syscall::dispatch::require_capability;
use crate::syscall::SyscallResult;

use super::primitives::copy;
use crate::security::hardening::memory_sanitization::secure_zero_slice;

/// `TPM_RC_POLICY_FAIL` as a format-1 code with the handle, parameter or
/// session number stripped: bits 8 to 11 carry that number, bit 6 says
/// which kind, and neither is part of the error.
const RC_POLICY_FAIL: u32 = 0x09D;
const RC_FORMAT1_ERROR_MASK: u32 = 0x0BF;

pub fn handle_machine_key(label_ptr: u64, label_len: u64, out_ptr: u64) -> SyscallResult {
    if let Err(e) = require_capability(Capability::Crypto) {
        return e;
    }
    let label = match copy::read_vec(label_ptr, label_len, LABEL_MAX) {
        Ok(v) => v,
        Err(e) => return e,
    };
    match derive(&label) {
        Ok(mut key) => {
            let written = copy::write(out_ptr, &key);
            secure_zero_slice(&mut key);
            written
        }
        Err(e) => {
            crate::log_warn!("[TPM] machine key: {}", e.as_str());
            crate::syscall::dispatch::errno(errno_for(e))
        }
    }
}

/// The errno says which of three very different things went wrong: no TPM to
/// ask, a TPM that refused because the machine is not in the state the key
/// belongs to, or a transport fault. A caller unlocking a volume shows the
/// user a different sentence for each.
fn errno_for(e: KeyError) -> i32 {
    match e {
        KeyError::Tpm(TpmError::NotPresent) => 19,
        KeyError::Tpm(TpmError::Timeout) => 110,
        KeyError::Tpm(TpmError::InvalidResponse) => 5,
        KeyError::Refused(rc) if rc & RC_FORMAT1_ERROR_MASK == RC_POLICY_FAIL => 13,
        KeyError::Refused(_) => 5,
        KeyError::BadLabel => 22,
    }
}
