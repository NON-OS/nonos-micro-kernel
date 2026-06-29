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

const LABEL: &[u8] = b"TLS 1.3, server CertificateVerify";

// RFC 8446 4.4.3: the server signs (0x20*64 || label || 0x00 ||
// Transcript-Hash(ClientHello..Certificate)) with the leaf key. `before_cv`
// is the running transcript snapshot taken BEFORE the CertificateVerify
// message is appended. `body` is the msg-15 body (after its 4-byte header).
pub fn verify_cert_verify(leaf: &[u8], before_cv: &[u8], body: &[u8]) -> bool {
    if body.len() < 4 {
        return false;
    }
    let scheme = u16::from_be_bytes([body[0], body[1]]);
    let siglen = u16::from_be_bytes([body[2], body[3]]) as usize;
    if 4 + siglen != body.len() {
        return false;
    }
    let sig = &body[4..4 + siglen];
    let Some(th) = super::hash_sha256::hash_sha256(before_cv) else {
        return false;
    };
    let mut blob = [0x20u8; 64 + 33 + 1 + 32];
    blob[64..64 + LABEL.len()].copy_from_slice(LABEL);
    blob[64 + LABEL.len()] = 0x00;
    blob[64 + LABEL.len() + 1..].copy_from_slice(&th);
    let Some(spki) = super::cert_spki::cert_spki(leaf) else {
        return false;
    };
    match scheme {
        0x0403 => ecdsa(spki, &blob, sig, 32),
        0x0503 => ecdsa(spki, &blob, sig, 48),
        0x0804 => rsa(1, 0, spki, sig, &blob),
        0x0805 => rsa(1, 1, spki, sig, &blob),
        0x0401 => rsa(0, 0, spki, sig, &blob),
        _ => false,
    }
}

fn rsa(scheme: u8, hashid: u8, spki: &[u8], sig: &[u8], blob: &[u8]) -> bool {
    if hashid == 1 {
        match super::hash_sha384::hash_sha384(blob) {
            Some(d) => super::verify_rsa::verify_rsa(scheme, hashid, spki, sig, &d),
            None => false,
        }
    } else {
        match super::hash_sha256::hash_sha256(blob) {
            Some(d) => super::verify_rsa::verify_rsa(scheme, hashid, spki, sig, &d),
            None => false,
        }
    }
}

fn ecdsa(spki: &[u8], blob: &[u8], sig: &[u8], width: usize) -> bool {
    let Some(pt) = super::spki_point::spki_point(spki) else {
        return false;
    };
    if width == 32 {
        if pt.len() != 65 {
            return false;
        }
        let Some(d) = super::hash_sha256::hash_sha256(blob) else {
            return false;
        };
        let mut pk = [0u8; 65];
        pk.copy_from_slice(pt);
        let mut raw = [0u8; 64];
        super::ecdsa_sig_raw::ecdsa_sig_raw(sig, 32, &mut raw)
            && super::verify_p256::verify_p256(&pk, &raw, &d)
    } else {
        if pt.len() != 97 {
            return false;
        }
        let Some(d) = super::hash_sha384::hash_sha384(blob) else {
            return false;
        };
        let mut pk = [0u8; 97];
        pk.copy_from_slice(pt);
        let mut raw = [0u8; 96];
        super::ecdsa_sig_raw::ecdsa_sig_raw(sig, 48, &mut raw)
            && super::verify_p384::verify_p384(&pk, &raw, &d)
    }
}
