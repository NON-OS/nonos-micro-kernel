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

extern crate alloc;

use crate::capabilities::Capability;
use crate::syscall::dispatch::{errno, require_capability};
use crate::syscall::SyscallResult;
use crate::usercopy::{copy_from_user, copy_to_user};

/// keccak256 over a user buffer. Pure hash, no key material.
///
/// Args: data ptr, len, out ptr (must be 32-byte), out_len (must == 32).
/// Returns 32 on success; negative errno otherwise.
pub fn handle_crypto_keccak256(data: u64, len: u64, out: u64, out_len: u64) -> SyscallResult {
    if let Err(e) = require_capability(Capability::Crypto) {
        return e;
    }
    if data == 0 || len == 0 || len > 1024 * 1024 {
        return errno(22);
    }
    if out == 0 || out_len != 32 {
        return errno(22);
    }
    let mut input = alloc::vec![0u8; len as usize];
    if copy_from_user(data, &mut input).is_err() {
        return errno(14);
    }
    let digest = crate::crypto::sha3::keccak256(&input);
    if copy_to_user(out, &digest).is_err() {
        return errno(14);
    }
    SyscallResult { value: 32, capability_consumed: false, audit_required: true }
}
