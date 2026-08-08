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

use super::concat::concatenate;
use super::types::RoutingInformation;
use crate::crypto::aes::Ctr64Be;
use crate::sphinx::constants::{ENCRYPTED_ROUTING_INFO_SIZE, STREAM_CIPHER_KEY_SIZE};
use alloc::vec;

/// XOR the assembled fields with this hop's keystream.
pub fn encrypt_routing_info(
    info: &RoutingInformation,
    key: &[u8; STREAM_CIPHER_KEY_SIZE],
) -> Option<[u8; ENCRYPTED_ROUTING_INFO_SIZE]> {
    let plain = concatenate(info);
    if plain.len() != ENCRYPTED_ROUTING_INFO_SIZE {
        return None;
    }
    let mut ks = vec![0u8; ENCRYPTED_ROUTING_INFO_SIZE];
    Ctr64Be::new(key, &[0u8; 16]).keystream(&mut ks);
    let mut out = [0u8; ENCRYPTED_ROUTING_INFO_SIZE];
    for (i, byte) in out.iter_mut().enumerate() {
        *byte = plain[i] ^ ks[i];
    }
    Some(out)
}
