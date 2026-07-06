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

//! The executable specification the differential proofs compare against.
//!
//! Each function restates, in the plainest possible Rust with the constants
//! written out literally, the contract that the Verus specifications in
//! `verification/verus` and the Lean models in `verification/lean` formalize.
//! The differential harnesses run the real kernel functions, included via
//! `#[path]`, against these over the input space (Kani over all inputs, fuzz
//! over samples), so any drift between the implementation and the
//! specification breaks the build instead of silently invalidating the
//! proofs upstream.

use crate::usercopy::error::UsercopyError;

// Capability tokens: verification/verus/src/capabilities.rs `has`, `grant`,
// `revoke`; verification/lean Nonos/Capability.lean and CapabilityBits.lean.

pub fn has(bits: u64, bit: u64) -> bool {
    bits & bit != 0
}

pub fn grant(bits: u64, bit: u64) -> u64 {
    bits | bit
}

pub fn revoke(bits: u64, bit: u64) -> u64 {
    bits & !bit
}

// User-copy range policy: verification/lean Nonos/Isolation.lean `Accepts`.
// The constants are the specification, restated literally.

pub const USER_SPACE_END: u64 = 0x0000_7FFF_FFFF_FFFF;
pub const MAX_COPY_SIZE: usize = 64 * 1024 * 1024;

pub fn check_range(addr: u64, len: usize) -> Result<Option<(u64, u64)>, UsercopyError> {
    if addr == 0 {
        return Err(UsercopyError::NullPointer);
    }
    if len > MAX_COPY_SIZE {
        return Err(UsercopyError::SizeTooLarge);
    }
    if len == 0 {
        return Ok(None);
    }
    let end = match addr.checked_add(len as u64 - 1) {
        Some(e) => e,
        None => return Err(UsercopyError::AddressOverflow),
    };
    if end > USER_SPACE_END {
        return Err(UsercopyError::InvalidAddress);
    }
    Ok(Some((addr & !0xFFF, end & !0xFFF)))
}

// Page-permission encoding: verification/verus/src/page_permissions.rs and
// verification/lean Nonos/Paging.lean. Permission bits on the left, PTE bits
// on the right, both written as literals.

pub fn pte_flags(perm: u32) -> u64 {
    let mut flags = 1u64; // present
    if perm & (1 << 1) != 0 {
        flags |= 1 << 1; // write -> writable
    }
    if perm & (1 << 3) != 0 {
        flags |= 1 << 2; // user -> user
    }
    if perm & (1 << 6) != 0 {
        flags |= 1 << 3; // write-through
    }
    if perm & (1 << 5) != 0 {
        flags |= 1 << 4; // no-cache -> cache-disable
    }
    if perm & (1 << 4) != 0 {
        flags |= 1 << 8; // global
    }
    if perm & (1 << 2) == 0 {
        flags |= 1u64 << 63; // no execute -> NX
    }
    flags
}

pub fn wx_violation(perm: u32) -> bool {
    perm & (1 << 1) != 0 && perm & (1 << 2) != 0
}
