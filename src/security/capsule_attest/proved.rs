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

use crate::security::dev_roots::Authority;

/// What a successful attestation established.
///
/// The two fields travel together everywhere. A measurement says what ran; the
/// authority says whose word that is. Separating them is how a system ends up
/// reporting locally built software as vendor-signed, so they are one value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Proved {
    pub measurement: [u8; 32],
    pub authority: Authority,
}

impl Proved {
    /// True when the shipped policy root vouched for this capsule. Callers
    /// that must not accept locally built code check this rather than
    /// inspecting the authority themselves.
    pub const fn is_vendor(&self) -> bool {
        self.authority.is_vendor()
    }
}
