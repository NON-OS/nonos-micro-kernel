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

use alloc::vec::Vec;

/// Wire format version. A verifier that does not recognise it must refuse
/// rather than parse optimistically: a document it half understands is worse
/// than one it rejects.
pub const DOC_VERSION: u32 = 1;

pub const DOC_MAGIC: &[u8; 8] = b"NONOSATT";

/// What the machine hands to whoever asked what it is running.
///
/// `registry_root` and `capsule_count` are carried in the clear for the
/// verifier's convenience, but their authority comes from the quote: the root
/// was folded into the qualifying data the TPM signed. A verifier recomputes
/// that binding rather than trusting these fields.
pub struct AttestationDoc {
    pub challenge: [u8; 32],
    pub registry_root: [u8; 32],
    pub capsule_count: u32,
    /// False once any running capsule could not be recorded. A verifier must
    /// treat a document with this clear as a statement that the machine no
    /// longer knows everything it is running.
    pub registry_complete: bool,
    /// The `TPMS_ATTEST` the TPM produced, byte for byte as signed.
    pub attest: Vec<u8>,
    pub signature: Vec<u8>,
}

impl AttestationDoc {
    /// Length-prefixed and versioned, so a verifier never has to guess where a
    /// field ends. Big-endian throughout to match the TPM structures it
    /// carries, rather than mixing conventions inside one document.
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(128 + self.attest.len() + self.signature.len());
        out.extend_from_slice(DOC_MAGIC);
        out.extend_from_slice(&DOC_VERSION.to_be_bytes());
        out.extend_from_slice(&self.challenge);
        out.extend_from_slice(&self.registry_root);
        out.extend_from_slice(&self.capsule_count.to_be_bytes());
        out.push(u8::from(self.registry_complete));
        out.extend_from_slice(&(self.attest.len() as u32).to_be_bytes());
        out.extend_from_slice(&self.attest);
        out.extend_from_slice(&(self.signature.len() as u32).to_be_bytes());
        out.extend_from_slice(&self.signature);
        out
    }
}
