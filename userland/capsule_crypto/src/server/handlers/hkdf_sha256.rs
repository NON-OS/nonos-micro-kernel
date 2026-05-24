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

use super::hmac_core::hmac_sha256;
use crate::protocol::{
    encode_response, Request, EINVAL, EMSGSIZE, HKDF_OUT_MAX, HKDF_PART_MAX, OP_HKDF_SHA256,
};

pub fn hkdf_sha256(req: Request<'_>) -> Vec<u8> {
    let (out_len, salt, ikm, info) = match parse(req.payload) {
        Some(v) => v,
        None => return encode_response(OP_HKDF_SHA256, req.flags, req.request_id, EINVAL, &[]),
    };
    if out_len == 0 || out_len > HKDF_OUT_MAX {
        return encode_response(OP_HKDF_SHA256, req.flags, req.request_id, EMSGSIZE, &[]);
    }
    if salt.len() > HKDF_PART_MAX || ikm.len() > HKDF_PART_MAX || info.len() > HKDF_PART_MAX {
        return encode_response(OP_HKDF_SHA256, req.flags, req.request_id, EMSGSIZE, &[]);
    }
    let prk = hmac_sha256(salt, ikm);
    let out = expand(&prk, info, out_len);
    encode_response(OP_HKDF_SHA256, req.flags, req.request_id, 0, &out)
}

fn parse(payload: &[u8]) -> Option<(usize, &[u8], &[u8], &[u8])> {
    if payload.len() < 8 {
        return None;
    }
    let out_len = u16::from_le_bytes([payload[0], payload[1]]) as usize;
    let sl = u16::from_le_bytes([payload[2], payload[3]]) as usize;
    let il = u16::from_le_bytes([payload[4], payload[5]]) as usize;
    let fl = u16::from_le_bytes([payload[6], payload[7]]) as usize;
    let end = 8usize.checked_add(sl)?.checked_add(il)?.checked_add(fl)?;
    if end != payload.len() {
        return None;
    }
    Some((out_len, &payload[8..8 + sl], &payload[8 + sl..8 + sl + il], &payload[8 + sl + il..end]))
}

fn expand(prk: &[u8; 32], info: &[u8], out_len: usize) -> Vec<u8> {
    let mut out = Vec::with_capacity(out_len);
    let mut t: Vec<u8> = Vec::new();
    for counter in 1..=div_ceil(out_len, 32) {
        let mut data = Vec::with_capacity(t.len() + info.len() + 1);
        data.extend_from_slice(&t);
        data.extend_from_slice(info);
        data.push(counter as u8);
        t = hmac_sha256(prk, &data).to_vec();
        out.extend_from_slice(&t);
    }
    out.truncate(out_len);
    out
}

fn div_ceil(n: usize, d: usize) -> usize {
    (n + d - 1) / d
}
