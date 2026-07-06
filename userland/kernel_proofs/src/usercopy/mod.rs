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
#[allow(dead_code, clippy::all)]
#[path = "../../../../src/usercopy/error.rs"]
pub mod error;

#[allow(dead_code, clippy::all)]
#[path = "../../../../src/usercopy/policy.rs"]
pub mod policy;

pub const USER_SPACE_END: u64 = policy::USER_SPACE_END;
pub const MAX_COPY_SIZE: usize = policy::MAX_COPY_SIZE;

// `check_range` returns a private `UserRange`; expose only accept/reject for the
// proofs so the invariant is stated without leaking the internal type.
pub fn accepts(addr: u64, len: usize) -> bool {
    policy::check_range(addr, len).is_ok()
}
