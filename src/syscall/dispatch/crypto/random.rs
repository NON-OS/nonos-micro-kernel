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
use crate::security::entropy_capsule::client as entropy_client;
use crate::security::entropy_capsule::EntropyCapsuleError;
use crate::syscall::dispatch::{errno, require_capability};
use crate::syscall::SyscallResult;
use crate::usercopy::copy_to_user;

// User-facing CryptoRandom. CAP_CRYPTO at the syscall gate, then
// routed to the entropy capsule. When the capsule is unavailable
// (dead/stale/transport/source failure) requests fall back to the
// kernel hardware RNG so a missing capsule never starves a caller of
// real entropy; caller and protocol errors are still surfaced.
pub fn handle_crypto_random(buf: u64, len: u64) -> SyscallResult {
    if let Err(e) = require_capability(Capability::Crypto) {
        return e;
    }
    if buf == 0 || len == 0 || len > 4096 {
        return errno(22);
    }
    let mut buffer = alloc::vec![0u8; len as usize];
    match entropy_client::get_random(&mut buffer) {
        Ok(n) if n == len as usize => deliver(buf, &buffer, len),
        Ok(_) => errno(5),
        Err(e) if is_caller_error(&e) => map_entropy_error(e),
        Err(_) => match crate::security::crypto::random::try_fill_random(&mut buffer) {
            Ok(()) => deliver(buf, &buffer, len),
            Err(_) => errno(5),
        },
    }
}

fn deliver(buf: u64, buffer: &[u8], len: u64) -> SyscallResult {
    if copy_to_user(buf, buffer).is_err() {
        return errno(14);
    }
    SyscallResult { value: len as i64, capability_consumed: false, audit_required: false }
}

fn is_caller_error(e: &EntropyCapsuleError) -> bool {
    matches!(
        e,
        EntropyCapsuleError::AccessDenied
            | EntropyCapsuleError::InvalidArgument
            | EntropyCapsuleError::OversizedRequest
            | EntropyCapsuleError::ProtocolMismatch
    )
}

fn map_entropy_error(err: EntropyCapsuleError) -> SyscallResult {
    match err {
        EntropyCapsuleError::AccessDenied => errno(13),
        EntropyCapsuleError::InvalidArgument => errno(22),
        EntropyCapsuleError::OversizedRequest => errno(90),
        EntropyCapsuleError::ProtocolMismatch => errno(71),
        EntropyCapsuleError::Dead => errno(19),
        EntropyCapsuleError::Stale => errno(116),
        EntropyCapsuleError::SourceFailure
        | EntropyCapsuleError::NoCallerPid
        | EntropyCapsuleError::TransportFailure => errno(5),
    }
}
