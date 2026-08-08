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

use super::types::DIGEST_BYTES;
use crate::crypto::aes::Ctr64Be;
use crate::crypto::hash::blake3;
use crate::surb::{candidates, SURB_KEY_BYTES};

/// Recover the fragment inside a reply.
///
/// What arrives is a tag saying which of our reply blocks the reply came back
/// on, then the fragment sealed under that block's key.
///
/// The acknowledgement a packet carries is not part of this. It sits in front
/// of everything else on the wire, but the gateway lifts it out and forwards
/// it before handing the rest over, so by the time a reply reaches us it is
/// already gone. Skipping its width here read the tag out of the middle of
/// the ciphertext, where it matched nothing, and every reply the far end sent
/// was dropped as though it had been meant for somebody else.
///
/// The tag is a digest of the key rather than the key itself, so it names one
/// of ours to us and nothing to anyone else. Matching on it means a reply is
/// opened with the one key that can open it, instead of trying each in turn
/// and treating whichever produces bytes as correct.
pub fn open_reply(payload: &[u8]) -> Option<Vec<u8>> {
    if payload.len() < DIGEST_BYTES {
        return None;
    }
    let digest = &payload[..DIGEST_BYTES];
    let sealed = &payload[DIGEST_BYTES..];

    let key = match_key(digest)?;
    let mut fragment = Vec::with_capacity(sealed.len());
    fragment.extend_from_slice(sealed);
    // The key opened one packet only, so the counter starts where it did when
    // the far end sealed it.
    Ctr64Be::new(&key, &[0u8; 16]).apply(&mut fragment);
    Some(fragment)
}

/// The reply block key whose digest is `digest`.
fn match_key(digest: &[u8]) -> Option<[u8; SURB_KEY_BYTES]> {
    for key in candidates() {
        if key == [0u8; SURB_KEY_BYTES] {
            continue;
        }
        let mut hashed = [0u8; DIGEST_BYTES];
        if blake3(&key, &mut hashed).is_err() {
            return None;
        }
        if hashed == digest {
            return Some(key);
        }
    }
    None
}
