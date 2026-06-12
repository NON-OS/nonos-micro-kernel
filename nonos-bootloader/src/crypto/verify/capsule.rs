// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::bytes::verify_signature_bytes;
use super::error::VerifyError;
use super::metadata::CapsuleMetadata;
use super::SIG_LEN;
use crate::crypto::keys::KeyId;

pub fn verify_signature_full(blob: &[u8], meta: &CapsuleMetadata) -> Result<KeyId, VerifyError> {
    let sig_end = meta.offset_sig.checked_add(meta.len_sig).ok_or(VerifyError::Bounds)?;
    let pay_end = meta.offset_payload.checked_add(meta.len_payload).ok_or(VerifyError::Bounds)?;
    if sig_end > blob.len() || pay_end > blob.len() {
        return Err(VerifyError::Bounds);
    }
    let signature_bytes = &blob[meta.offset_sig..sig_end];
    let payload_bytes = &blob[meta.offset_payload..pay_end];
    if signature_bytes.len() != SIG_LEN {
        return Err(VerifyError::MalformedSignature);
    }
    if signature_bytes.iter().all(|&b| b == 0) {
        return Err(VerifyError::MalformedSignature);
    }
    verify_signature_bytes(payload_bytes, signature_bytes)
}
