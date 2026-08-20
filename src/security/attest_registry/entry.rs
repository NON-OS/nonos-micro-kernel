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

/// One capsule that passed the spawn gate and has not exited.
///
/// `measurement` is the value the proof was checked against, taken from the
/// verifier rather than recomputed, so what is attested is exactly what was
/// verified. `caps` is recorded alongside because a measurement alone does not
/// describe what the program was permitted to do, and a remote party checking
/// an attestation cares about both.
#[derive(Clone, Copy)]
pub struct AttestedCapsule {
    pub pid: u32,
    pub measurement: [u8; 32],
    pub caps: u64,
    /// Whose policy tree proved this capsule. Folded into the root, so an
    /// attestation cannot present locally built software as shipped software.
    pub authority: Authority,
}

impl AttestedCapsule {
    pub const fn empty() -> Self {
        Self { pid: 0, measurement: [0u8; 32], caps: 0, authority: Authority::Vendor }
    }

    /// Bytes folded into the registry root, in a fixed order so the digest is
    /// reproducible by a verifier that has the same set.
    ///
    /// The authority byte is inside the digest rather than reported beside it.
    /// A verifier comparing two machines running identical measurements under
    /// different authorities must see two different roots, or the distinction
    /// is decorative.
    pub fn digest_input(&self) -> [u8; 45] {
        let mut out = [0u8; 45];
        out[..4].copy_from_slice(&self.pid.to_be_bytes());
        out[4..36].copy_from_slice(&self.measurement);
        out[36..44].copy_from_slice(&self.caps.to_be_bytes());
        out[44] = match self.authority {
            Authority::Vendor => 0,
            // Slot is included: two developer keys are not interchangeable.
            Authority::Developer(slot) => 1 + slot,
        };
        out
    }
}
