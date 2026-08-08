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

use super::constants::SUITE_AES128_GCM_SHA256;

pub fn seal(
    suite: u16,
    key: &[u8; 32],
    iv: &[u8; 12],
    seq: u64,
    inner_type: u8,
    body: &[u8],
) -> Option<Vec<u8>> {
    let mut plain = Vec::with_capacity(body.len() + 1);
    plain.extend_from_slice(body);
    plain.push(inner_type);
    let mut head = Vec::with_capacity(5);
    head.push(23);
    super::push::u16(&mut head, 0x0303);
    super::push::u16(&mut head, plain.len() as u16 + 16);
    let nonce = super::nonce::nonce(iv, seq);
    if suite == SUITE_AES128_GCM_SHA256 {
        let mut k = [0u8; 16];
        k.copy_from_slice(&key[..16]);
        let ct = super::aes_gcm::seal(&k, &nonce, &head, &plain);
        head.extend_from_slice(&ct);
        return Some(head);
    }
    let frame = super::aad_frame::aad_frame(&head, &plain);
    let mut ct = alloc::vec![0u8; plain.len() + 16];
    let n = nonos_libc::crypto_encrypt_aad(
        0,
        key.as_ptr(),
        nonce.as_ptr(),
        frame.as_ptr(),
        frame.len(),
        ct.as_mut_ptr(),
    );
    if n != ct.len() as i64 {
        return None;
    }
    head.extend_from_slice(&ct);
    Some(head)
}
