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

use super::types::{ACK_IV_BYTES, ACK_PLAINTEXT_BYTES, FRAG_ID_BYTES};
use crate::crypto::aes::Ctr64Be;
use crate::crypto::random::fill_random;
use crate::crypto::types::CryptoError;

/// What an acknowledgement carries: a random prefix, then the fragment it
/// names, encrypted under it.
///
/// The fragment id is what we recognise our own ack by when it returns. It is
/// encrypted so that no hop along the way can tell two acks of ours apart,
/// which is what would otherwise link the packets they belong to.
pub fn ack_plaintext(
    ack_key: &[u8; 16],
    frag_id: [u8; FRAG_ID_BYTES],
) -> Result<[u8; ACK_PLAINTEXT_BYTES], CryptoError> {
    let mut out = [0u8; ACK_PLAINTEXT_BYTES];
    fill_random(&mut out[..ACK_IV_BYTES])?;
    out[ACK_IV_BYTES..].copy_from_slice(&frag_id);
    let mut iv = [0u8; ACK_IV_BYTES];
    iv.copy_from_slice(&out[..ACK_IV_BYTES]);
    Ctr64Be::new(ack_key, &iv).apply(&mut out[ACK_IV_BYTES..]);
    Ok(out)
}
