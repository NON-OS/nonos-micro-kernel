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

//! The one place this module talks to the TPM.
//!
//! Every command in the derivation goes through here, which is what makes the
//! safety argument checkable: it is one `unsafe` block with one claim to
//! verify, rather than six call sites each making it separately.

use alloc::vec::Vec;
use core::ops::Deref;

use super::error::KeyError;
use crate::security::hardening::memory_sanitization::secure_zero_slice;
use crate::security::tpm::crb::transact;

/// Longer than any response this module can provoke. The largest is the
/// create, whose public area and private blob together stay well inside this;
/// `transact` refuses to overrun the buffer rather than truncating into it.
const RESPONSE_MAX: usize = 1024;

/// A TPM response that wipes itself when dropped. For the HMAC it is the
/// root key, so the bytes do not stay in freed memory after parsing.
pub(super) struct Response(Vec<u8>);

impl Deref for Response {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.0
    }
}

impl Drop for Response {
    fn drop(&mut self) {
        secure_zero_slice(&mut self.0);
    }
}

pub(super) fn run(cmd: &[u8]) -> Result<Response, KeyError> {
    let mut buf = [0u8; RESPONSE_MAX];
    /*
     * SAFETY: eK@nonos.systems - every command built by this module creates
     * or flushes a transient object, starts a session, or reads one. None
     * writes NV storage and none names a persistent handle, so a failure
     * here cannot leave the TPM in a state that outlives the boot.
     */
    let len = unsafe { transact(cmd, &mut buf) };
    let response = len.map(|n| Response(buf[..n].to_vec())).map_err(KeyError::from);
    secure_zero_slice(&mut buf);
    response
}
