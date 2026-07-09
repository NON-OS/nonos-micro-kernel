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

// The real user-copy bounds policy plus a thin public surface for the proofs.
#[path = "../../../../src/usercopy/error.rs"]
pub mod error;

#[path = "../../../../src/usercopy/policy.rs"]
pub mod policy;

pub const USER_SPACE_END: u64 = policy::USER_SPACE_END;
pub const MAX_COPY_SIZE: usize = policy::MAX_COPY_SIZE;
pub const PAGE_SIZE: u64 = policy::PAGE_SIZE;

// `check_range` returns a private `UserRange`; expose accept/reject and the
// accepted page range so the proofs can state the invariant without leaking the
// internal type.
pub fn accepts(addr: u64, len: usize) -> bool {
    policy::check_range(addr, len).is_ok()
}

// The page-aligned range covered by an accepted non-empty copy, or None.
pub fn accepted_range(addr: u64, len: usize) -> Option<(u64, u64)> {
    match policy::check_range(addr, len) {
        Ok(Some(range)) => Some((range.start_page, range.end_page)),
        _ => None,
    }
}

// The full decision, error variant included, for the functional differential
// against the executable spec. The private `UserRange` is flattened to its
// page pair; nothing else is changed.
pub fn check(addr: u64, len: usize) -> Result<Option<(u64, u64)>, error::UsercopyError> {
    policy::check_range(addr, len).map(|r| r.map(|range| (range.start_page, range.end_page)))
}
