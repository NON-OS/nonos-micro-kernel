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

//! `MkDevRootRequest` and `MkDevRootConfirm`: let this machine run software
//! written on it.
//!
//! Two calls, because one would let a capsule widen what the machine executes
//! without a human involved. The request prints a code the kernel writes
//! straight to the console; the confirm needs that code back. A capsule can
//! ask, and cannot read the answer to its own question.
//!
//! The effect ends at the next boot, like everything else here.

use super::errnos::ERRNO_FAULT;
use crate::capabilities::caps_to_bits;
use crate::security::dev_roots::{confirm_dev_root, request_dev_root, Authority};
use crate::syscall::caps::current_caps_or_default;

const ROOT_LEN: usize = 32;

/// Ask to enrol `root`. Shows the user a confirmation code. Returns 0 when the
/// request was accepted for confirmation, not when anything was enrolled.
pub fn sys_dev_root_request(root_ptr: u64) -> i64 {
    let mut root = [0u8; ROOT_LEN];
    if crate::usercopy::copy_from_user(root_ptr, &mut root).is_err() {
        return ERRNO_FAULT;
    }
    // Read from the live token: the caller does not get to describe its own
    // authority.
    let caps = caps_to_bits(&current_caps_or_default().permissions);
    match request_dev_root(caps, root) {
        Ok(()) => 0,
        Err(e) => {
            crate::sys::serial::print(b"[DEV-ROOT] request refused: ");
            crate::sys::serial::println(e.as_str().as_bytes());
            e.to_errno()
        }
    }
}

/// Complete the pending enrolment with the code the user read off the console.
/// Returns the developer slot on success.
pub fn sys_dev_root_confirm(answer: u64) -> i64 {
    let caps = caps_to_bits(&current_caps_or_default().permissions);
    // Narrowed before use: a 64-bit argument cannot be allowed to compare
    // equal to a 32-bit challenge by carrying bits the challenge never had.
    let answer = answer as u32;
    match confirm_dev_root(caps, answer) {
        Ok(Authority::Developer(slot)) => slot as i64,
        // The vendor authority is never the result of an enrolment.
        Ok(Authority::Vendor) => ERRNO_FAULT,
        Err(e) => {
            crate::sys::serial::print(b"[DEV-ROOT] confirm refused: ");
            crate::sys::serial::println(e.as_str().as_bytes());
            e.to_errno()
        }
    }
}
