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
use crate::syscall::dispatch::{errno, require_capability};
use crate::syscall::SyscallResult;

const MAX_FRAME: usize = 784;
const MAX_OUT: usize = 512;

pub fn handle_hkdf_sha256(
    frame_ptr: u64,
    frame_len: u64,
    out_ptr: u64,
    out_len: u64,
) -> SyscallResult {
    if let Err(e) = require_capability(Capability::Crypto) {
        return e;
    }
    if out_len == 0 || out_len as usize > MAX_OUT {
        return errno(22);
    }
    let frame = match copy::read_vec(frame_ptr, frame_len, MAX_FRAME) {
        Ok(v) => v,
        Err(e) => return e,
    };
    match client::hkdf_sha256(&frame, out_len as usize) {
        Ok(out) => copy::write(out_ptr, &out),
        Err(e) => map_capsule_error(e, CryptoErrorContext::Digest),
    }
}
