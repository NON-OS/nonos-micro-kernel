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

use super::super::addr20::addr20;
use super::super::consts::PRICE_KIND_FREE;
use super::super::discover::payment_port;
use super::super::fields::InstallReq;
use super::super::pay_call::request_payment;
use super::super::word32::word32;
use crate::protocol::{encode_response, Request, EAGAIN, EINVAL};

pub fn install(req: Request<'_>) -> Vec<u8> {
    const HDR: usize = 4 + 4 + 1 + 32 + 20 + 32 + 32;
    if req.payload.len() != HDR {
        return encode_response(req.seq, EINVAL, &[]);
    }
    let p = req.payload;
    let r = InstallReq {
        owner_pid: u32::from_le_bytes([p[0], p[1], p[2], p[3]]),
        wallet_id: u32::from_le_bytes([p[4], p[5], p[6], p[7]]),
        price_kind: p[8],
        capsule_id: word32(p, 9),
        publisher: addr20(p, 41),
        amount: word32(p, 61),
        receipt_type: word32(p, 93),
    };
    if r.price_kind == PRICE_KIND_FREE {
        return encode_response(req.seq, 0, &[0u8; 32]);
    }
    let port = match payment_port() {
        Some(x) => x,
        None => return encode_response(req.seq, EAGAIN, &[]),
    };
    match request_payment(port, &r) {
        Ok(struct_hash) => encode_response(req.seq, 0, &struct_hash),
        Err(e) => encode_response(req.seq, e, &[]),
    }
}
