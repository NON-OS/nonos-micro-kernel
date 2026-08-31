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

/// Domain separator. Without one, a digest computed here could be replayed as
/// a digest computed for some other purpose over the same bytes.
const BIND_CONTEXT: &[u8] = b"nonos.attest.bind.v1";

/// Fold the challenge and the registry root into the value handed to the TPM
/// as `qualifyingData`.
///
/// This is the load-bearing step of the whole design. A TPM quote signs two
/// things: the PCR values, and whatever the caller passed as qualifying data.
/// It knows nothing about capsules. If the registry root travelled beside the
/// quote instead of inside it, the root would be unsigned, and anyone could
/// attach an honest quote to a fabricated list of what is running, which is
/// precisely the claim the document exists to make.
///
/// Binding it here means the TPM's signature covers the root transitively: a
/// verifier recomputes this digest from the challenge it issued and the root
/// it was shown, and any substitution of either produces a different value
/// than the one the TPM signed.
pub fn qualifying_data(challenge: &[u8; 32], registry_root: &[u8; 32]) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(BIND_CONTEXT);
    hasher.update(challenge);
    hasher.update(registry_root);
    *hasher.finalize().as_bytes()
}
