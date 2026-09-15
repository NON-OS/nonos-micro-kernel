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

//! A key only this machine, booted this way, can produce.
//!
//! Stored nowhere. The kernel asks the TPM for an HMAC over the label under an
//! object derived from the owner seed and the boot PCRs, so the same label
//! gives the same 32 bytes on every boot of this machine and different bytes
//! on any other, or under any other kernel. It is the wrapping key for
//! anything a capsule wants to find again after a reboot.

use crate::syscall::{call_raw, N_CRYPTO_MACHINE_KEY};

/// Longer labels are refused by the kernel with EINVAL.
pub const MACHINE_KEY_LABEL_MAX: usize = 64;

/// The TPM refused because the boot state is not the one the key belongs to.
pub const MACHINE_KEY_WRONG_STATE: i64 = -13;
/// There is no TPM to ask.
pub const MACHINE_KEY_NO_TPM: i64 = -19;

#[no_mangle]
pub extern "C" fn crypto_machine_key(label: *const u8, label_len: usize, out: *mut u8) -> i64 {
    call_raw(N_CRYPTO_MACHINE_KEY, [label as u64, label_len as u64, out as u64, 0, 0, 0])
}

/// The safe form: 32 bytes for a label, or the errno the kernel gave.
pub fn machine_key(label: &[u8]) -> Result<[u8; 32], i64> {
    let mut out = [0u8; 32];
    let rc = crypto_machine_key(label.as_ptr(), label.len(), out.as_mut_ptr());
    if rc < 0 {
        return Err(rc);
    }
    Ok(out)
}
