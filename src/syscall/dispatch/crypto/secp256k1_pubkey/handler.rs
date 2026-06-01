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

use core::sync::atomic::{compiler_fence, Ordering};

use crate::capabilities::Capability;
use crate::syscall::dispatch::{errno, require_capability};
use crate::syscall::SyscallResult;
use crate::usercopy::{copy_from_user, copy_to_user};

pub fn handle_crypto_secp256k1_pubkey(sk_ptr: u64, out: u64) -> SyscallResult {
    if let Err(e) = require_capability(Capability::Crypto) {
        return e;
    }
    if sk_ptr == 0 || out == 0 {
        return errno(22);
    }
    let mut sk = [0u8; 32];
    if copy_from_user(sk_ptr, &mut sk).is_err() {
        for b in sk.iter_mut() {
            unsafe { core::ptr::write_volatile(b, 0) };
        }
        compiler_fence(Ordering::SeqCst);
        return errno(14);
    }
    let pk = crate::crypto::secp256k1::public_key_from_secret(&sk);
    for b in sk.iter_mut() {
        unsafe { core::ptr::write_volatile(b, 0) };
    }
    compiler_fence(Ordering::SeqCst);
    let pk = match pk {
        Some(p) => p,
        None => return errno(22),
    };
    if copy_to_user(out, &pk).is_err() {
        return errno(14);
    }
    SyscallResult { value: 65, capability_consumed: false, audit_required: true }
}
