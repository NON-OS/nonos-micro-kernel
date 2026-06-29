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
use rsa::pkcs8::DecodePublicKey;
use rsa::pss::Pss;
use rsa::{Pkcs1v15Sign, RsaPublicKey};
use sha2::{Sha256, Sha384, Sha512};

use crate::protocol::{encode_response, Request, EBADMSG, EINVAL, OP_RSA_VERIFY};

pub fn rsa_verify(req: Request<'_>) -> Vec<u8> {
    let reply = |status: i32| encode_response(OP_RSA_VERIFY, req.flags, req.request_id, status, &[]);
    let p = req.payload;
    if p.len() < 6 {
        return reply(EINVAL);
    }
    let scheme = p[0];
    let hashid = p[1];
    let spki_len = u16::from_le_bytes([p[2], p[3]]) as usize;
    let mut o = 4;
    if p.len() < o + spki_len + 2 {
        return reply(EINVAL);
    }
    let spki = &p[o..o + spki_len];
    o += spki_len;
    let sig_len = u16::from_le_bytes([p[o], p[o + 1]]) as usize;
    o += 2;
    if p.len() < o + sig_len {
        return reply(EINVAL);
    }
    let sig = &p[o..o + sig_len];
    o += sig_len;
    let digest = &p[o..];
    let want = match hashid {
        0 => 32,
        1 => 48,
        2 => 64,
        _ => return reply(EINVAL),
    };
    if digest.len() != want {
        return reply(EINVAL);
    }
    let key = match RsaPublicKey::from_public_key_der(spki) {
        Ok(k) => k,
        Err(_) => return reply(EINVAL),
    };
    let ok = match (scheme, hashid) {
        (0, 0) => key.verify(Pkcs1v15Sign::new::<Sha256>(), digest, sig).is_ok(),
        (0, 1) => key.verify(Pkcs1v15Sign::new::<Sha384>(), digest, sig).is_ok(),
        (0, 2) => key.verify(Pkcs1v15Sign::new::<Sha512>(), digest, sig).is_ok(),
        (1, 0) => key.verify(Pss::new::<Sha256>(), digest, sig).is_ok(),
        (1, 1) => key.verify(Pss::new::<Sha384>(), digest, sig).is_ok(),
        (1, 2) => key.verify(Pss::new::<Sha512>(), digest, sig).is_ok(),
        _ => false,
    };
    reply(if ok { 0 } else { EBADMSG })
}
