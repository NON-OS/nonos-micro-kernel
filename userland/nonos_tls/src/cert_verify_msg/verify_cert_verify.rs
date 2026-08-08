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

use crate::cert_verify_msg::{constants, verify_ecdsa, verify_rsa};

pub fn verify_cert_verify(leaf: &[u8], before_cv: &[u8], body: &[u8]) -> bool {
    if body.len() < 4 {
        return false;
    }
    let scheme = u16::from_be_bytes([body[0], body[1]]);
    let siglen = u16::from_be_bytes([body[2], body[3]]) as usize;
    if 4 + siglen != body.len() {
        return false;
    }
    let Some(th) = super::super::hash_sha256::hash_sha256(before_cv) else {
        return false;
    };
    let mut blob = [0x20u8; 64 + 33 + 1 + 32];
    blob[64..64 + constants::LABEL.len()].copy_from_slice(constants::LABEL);
    blob[64 + constants::LABEL.len()] = 0x00;
    blob[64 + constants::LABEL.len() + 1..].copy_from_slice(&th);
    let Some(spki) = super::super::cert_spki::cert_spki(leaf) else {
        return false;
    };
    match scheme {
        0x0403 => verify_ecdsa::verify_ecdsa(spki, &blob, &body[4..], 32),
        0x0503 => verify_ecdsa::verify_ecdsa(spki, &blob, &body[4..], 48),
        0x0804 => verify_rsa::verify_rsa(1, 0, spki, &body[4..], &blob),
        0x0805 => verify_rsa::verify_rsa(1, 1, spki, &body[4..], &blob),
        0x0401 => verify_rsa::verify_rsa(0, 0, spki, &body[4..], &blob),
        _ => false,
    }
}
