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

use super::copy;
use crate::capabilities::Capability;
use crate::security::crypto_capsule::client;
use crate::syscall::dispatch::crypto::error::{map_capsule_error, CryptoErrorContext};
use crate::syscall::dispatch::require_capability;
use crate::syscall::SyscallResult;
use alloc::vec::Vec;

const MAX_KEY: usize = 256;
const MAX_DATA: usize = 65536;

pub fn handle_hmac_sha256(
    key_ptr: u64,
    key_len: u64,
    data_ptr: u64,
    data_len: u64,
    out_ptr: u64,
) -> SyscallResult {
    if let Err(e) = require_capability(Capability::Crypto) {
        return e;
    }
    let key = match read_optional(key_ptr, key_len, MAX_KEY) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let data = match read_optional(data_ptr, data_len, MAX_DATA) {
        Ok(v) => v,
        Err(e) => return e,
    };
    match client::hmac_sha256(&key, &data) {
        Ok(out) => copy::write(out_ptr, &out),
        Err(e) => map_capsule_error(e, CryptoErrorContext::Digest),
    }
}

fn read_optional(ptr: u64, len: u64, max: usize) -> Result<Vec<u8>, SyscallResult> {
    if len == 0 {
        return Ok(Vec::new());
    }
    copy::read_vec(ptr, len, max)
}
