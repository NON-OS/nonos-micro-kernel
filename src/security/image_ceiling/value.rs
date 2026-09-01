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

use crate::capabilities::Capability;

// Staged by build.rs: the image's ceiling file when it ships one, otherwise the
// unset default that reads back as unrestricted. An image restricts its own
// authority by dropping nonos-data/trust/policy/image_capability_ceiling.bin;
// absence is not a build error, it is the ordinary case.
const BAKED: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/image_capability_ceiling.bin"));

/// Const because `ceiling` sits on the token mint path.
const fn unrestricted() -> u64 {
    let caps = Capability::all();
    let mut bits = 0u64;
    let mut i = 0;
    while i < caps.len() {
        bits |= caps[i].bit();
        i += 1;
    }
    bits
}

const UNRESTRICTED: u64 = unrestricted();

/// The most authority any capsule in this image may hold. Baked in beside the
/// policy root, so nothing the image runs can raise it.
///
/// Malformed or absent reads as unrestricted, not as zero. Zero is a machine
/// that cannot spawn anything, and it would make a corrupt file look like a
/// deliberately locked image.
pub const fn ceiling() -> u64 {
    if BAKED.len() != 8 {
        return UNRESTRICTED;
    }
    let declared = u64::from_le_bytes([
        BAKED[0], BAKED[1], BAKED[2], BAKED[3], BAKED[4], BAKED[5], BAKED[6], BAKED[7],
    ]);
    if declared == 0 {
        return UNRESTRICTED;
    }
    declared & UNRESTRICTED
}

/// Lets a verifier tell a deliberate restriction from a default.
pub const fn is_restricted() -> bool {
    ceiling() != UNRESTRICTED
}
