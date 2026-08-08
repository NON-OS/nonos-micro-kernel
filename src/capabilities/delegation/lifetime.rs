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

//! When a delegation expires, holding no token so it can be included by
//! `mechanism_proofs` and checked against `Nonos.Delegation`.
//! `create_delegation` delegates here.
//!
//! A delegation is signed by the kernel key, so one that outlives its parent
//! verifies everywhere for as long as it lasts. The meet below is what stops
//! that: whatever the caller asks for, the answer is never later than the
//! parent's own expiry.

/// The expiry of a delegation: the earlier of what the caller asked for and
/// what the parent has left.
///
/// `None` means no expiry. A parent without one imposes no bound, so the
/// requested value stands; a parent with one always bounds the result, even
/// when the caller asked for nothing.
pub(crate) fn delegation_expiry(requested: Option<u64>, parent: Option<u64>) -> Option<u64> {
    match parent {
        Some(parent_exp) => Some(match requested {
            Some(e) => {
                if e < parent_exp {
                    e
                } else {
                    parent_exp
                }
            }
            None => parent_exp,
        }),
        None => requested,
    }
}
