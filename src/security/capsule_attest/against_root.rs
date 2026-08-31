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

/// Verify a capsule's proof against one specific root.
///
/// The root is a parameter rather than a lookup. That is the whole point: the
/// verification is identical whoever owns the tree, so a capsule built on this
/// machine clears exactly the bar a shipped one does. Only membership differs.
///
/// Returns the measurement the proof was checked against, so a caller records
/// what was verified rather than recomputing it and hoping the two agree.
pub(super) fn verify(
    trailer: &[u8],
    elf: &[u8],
    granted_caps: u64,
    root: &[u8; 32],
) -> Result<[u8; 32], AttestError> {
    #[cfg(feature = "nonos-stark-attest")]
    {
        super::stark::verify_against(trailer, elf, granted_caps, root)
    }
    #[cfg(not(feature = "nonos-stark-attest"))]
    {
        use super::layout::POLICY_EPOCH;
        use super::trailer::parse;
        use crate::crypto::zk_kernel::verify_enrolled;

        let proof = parse(trailer)?;
        let capsule_hash = *blake3::hash(elf).as_bytes();
        let mut ctx = [0u8; 48];
        ctx[..32].copy_from_slice(&capsule_hash);
        ctx[32..40].copy_from_slice(&granted_caps.to_be_bytes());
        ctx[40..48].copy_from_slice(&POLICY_EPOCH.to_be_bytes());

        if verify_enrolled(&proof, root, &ctx) {
            Ok(capsule_hash)
        } else {
            Err(AttestError::Rejected)
        }
    }
}
