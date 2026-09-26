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

use super::error::AttestError;

/// The two proof shapes, told apart by their own first eight bytes.
const STARK_MAGIC: &[u8; 8] = b"NZKSTRK1";

/// Verify a capsule's proof against one specific root.
pub(super) fn verify(
    trailer: &[u8],
    elf: &[u8],
    granted_caps: u64,
    root: &[u8; 32],
) -> Result<[u8; 32], AttestError> {
    if trailer.len() >= 8 && &trailer[0..8] == STARK_MAGIC {
        return stark(trailer, elf, granted_caps, root);
    }
    super::against_pedersen::verify(trailer, elf, granted_caps, root)
}

#[cfg(feature = "nonos-stark-attest")]
fn stark(
    trailer: &[u8],
    elf: &[u8],
    granted_caps: u64,
    root: &[u8; 32],
) -> Result<[u8; 32], AttestError> {
    super::stark::verify_against(trailer, elf, granted_caps, root)
}

/// A build without the STARK verifier cannot check a STARK trailer, and saying
/// so is the only safe answer: the alternative is falling through to the other
/// parser, which would refuse for the wrong reason.
#[cfg(not(feature = "nonos-stark-attest"))]
fn stark(_: &[u8], _: &[u8], _: u64, _: &[u8; 32]) -> Result<[u8; 32], AttestError> {
    Err(AttestError::Rejected)
}

