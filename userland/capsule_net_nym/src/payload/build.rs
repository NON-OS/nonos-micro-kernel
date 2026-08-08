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

use super::shared::packet_shared_key;
use crate::crypto::aes::Ctr64Be;
use crate::crypto::types::CryptoError;

/// Assemble what one packet carries.
///
/// A recipient reads this in a fixed order and cannot be told where one part
/// ends and the next begins, so the order is the contract: the ack it must
/// forward, the public half of a key agreed just for this packet, then the
/// fragment sealed under what that agreement produced.
///
/// The fragment is encrypted for the recipient specifically. Every hop before
/// it already peeled its own layer off the packet, but the last one would
/// otherwise read the message in the clear, and the recipient is the only
/// party meant to.
pub fn build_payload(
    ack_bytes: &[u8],
    recipient_encryption_key: &[u8; 32],
    fragment: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    let (ephemeral_public, key) = packet_shared_key(recipient_encryption_key)?;

    let mut sealed = Vec::with_capacity(fragment.len());
    sealed.extend_from_slice(fragment);
    // The key is used for exactly one packet, so a counter starting at zero
    // is never reused with it.
    Ctr64Be::new(&key, &[0u8; 16]).apply(&mut sealed);

    let mut out = Vec::with_capacity(ack_bytes.len() + ephemeral_public.len() + sealed.len());
    out.extend_from_slice(ack_bytes);
    out.extend_from_slice(&ephemeral_public);
    out.extend_from_slice(&sealed);
    Ok(out)
}
