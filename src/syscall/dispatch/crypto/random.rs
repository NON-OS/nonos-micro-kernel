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
use crate::usercopy::copy_to_user;

// User-facing CryptoRandom. CAP_CRYPTO at the syscall gate, then served from
// the kernel's ChaCha generator, which the entropy capsule seeds and reseeds.
// The capsule stays the root of the entropy story without sitting on the
// per-call path: routing every request through its inbox as an IPC round trip
// put millions of messages a minute onto one core, and the whole system,
// networking first, queued behind the RNG. When no seed has ever been obtained
// and the capsule cannot provide one, requests fall back to the kernel
// hardware RNG so a missing capsule never starves a caller of real entropy.
pub fn handle_crypto_random(buf: u64, len: u64) -> SyscallResult {
    if let Err(e) = require_capability(Capability::Crypto) {
        return e;
    }
    if buf == 0 || len == 0 || len > 4096 {
        return errno(22);
    }
    let mut buffer = alloc::vec![0u8; len as usize];
    if crate::security::entropy_capsule::fast::fill(&mut buffer) {
        return deliver(buf, &buffer, len);
    }
    match crate::security::crypto::random::try_fill_random(&mut buffer) {
        Ok(()) => deliver(buf, &buffer, len),
        Err(_) => errno(5),
    }
}

fn deliver(buf: u64, buffer: &[u8], len: u64) -> SyscallResult {
    if copy_to_user(buf, buffer).is_err() {
        return errno(14);
    }
    SyscallResult { value: len as i64, capability_consumed: false, audit_required: false }
}

