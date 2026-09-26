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

//! The sixteen bytes behind AT_RANDOM.

/// Sixteen bytes a C runtime turns into its stack guard.
pub const RANDOM_LEN: u64 = 16;

/// Entropy comes from the system, never from a constant: a fixed value here
/// would make every guest's stack guard the same, and a guard an attacker can
/// predict is not a guard.
pub fn random() -> Option<[u8; RANDOM_LEN as usize]> {
    let mut out = [0u8; RANDOM_LEN as usize];
    match nonos_libc::crypto_random(out.as_mut_ptr(), out.len()) {
        n if n < 0 => None,
        _ => Some(out),
    }
}
